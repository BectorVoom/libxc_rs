//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1426/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1426<F: Float>(t2371: F, t5801: F, t117: F, t13514: F, t1459: F, t1461: F, t18190: F, t18204: F, t18208: F, t1916: F, t1918: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F) -> F {
    let t18211 = t5801 * t2371;
    let t18214 = t117 * t13514;
    let t18217 = F::cast_from(12.0_f64) * t1459 * t5802 + F::cast_from(6.0_f64) * t1459 * t5805 + F::cast_from(6.0_f64) * t1461 * t5795 + t18190 * t573 + F::cast_from(6.0_f64) * t18204 * t572 + F::cast_from(12.0_f64) * t18208 * t572 + F::cast_from(6.0_f64) * t18211 * t572 + F::cast_from(3.0_f64) * t18214 * t572 + F::cast_from(6.0_f64) * t1916 * t4162 + F::cast_from(3.0_f64) * t1916 * t4165 + F::cast_from(3.0_f64) * t1918 * t4158;
    t18217
}
