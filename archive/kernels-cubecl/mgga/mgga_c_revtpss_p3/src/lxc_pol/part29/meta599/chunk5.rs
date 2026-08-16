//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2044/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2044<F: Float>(t102009: F, t102058: F, t102791: F, t103873: F, t103917: F, t103956: F, t103999: F, t104038: F, t101725: F, t101761: F, t117: F, t13514: F, t1459: F, t1518: F, t18190: F, t18204: F, t18208: F, t18211: F, t1916: F, t2113: F, t2115: F, t26733: F, t26740: F, t28974: F, t28987: F, t28990: F, t4162: F, t4292: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t7547: F, t7553: F, t7557: F, t8118: F, t96640: F, param_d: F) -> (F, F) {
    let t104041 = t102009 + t102058 + t102791 + t103873 + t103917 + t103956 + t103999 + t104038;
    let t104054 = F::cast_from(12.0_f64) * t572 * t101725 * t1518 + F::cast_from(12.0_f64) * t572 * t28974 * t4292 + F::cast_from(6.0_f64) * t572 * t7553 * t13514 + F::cast_from(12.0_f64) * t1459 * t28987 + F::cast_from(3.0_f64) * t1916 * t26740 + F::cast_from(6.0_f64) * t1459 * t28990 + F::cast_from(12.0_f64) * t7547 * t5802 + F::cast_from(6.0_f64) * t572 * t96640 * t1518 + F::cast_from(12.0_f64) * t572 * t26733 * t4292 + F::cast_from(12.0_f64) * t2113 * t18208 + F::cast_from(3.0_f64) * t572 * t117 * t101761 + F::cast_from(6.0_f64) * t2113 * t18204 + param_d * t104041 * t573 + F::cast_from(6.0_f64) * t2113 * t18211 + F::cast_from(6.0_f64) * t5795 * t7557 + F::cast_from(6.0_f64) * t8118 * t4162 + F::cast_from(3.0_f64) * t18190 * t2115 + F::cast_from(6.0_f64) * t7547 * t5805;
    (t104041, t104054)
}
