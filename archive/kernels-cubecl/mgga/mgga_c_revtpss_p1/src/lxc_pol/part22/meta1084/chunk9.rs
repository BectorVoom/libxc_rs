//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3934/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3934<F: Float>(t61014: F, t75451: F, t75676: F, t75714: F, t1455: F, t6951: F, t1464: F, t6936: F, t116: F, t13514: F, t1459: F, t1461: F, t18204: F, t18211: F, t18214: F, t1916: F, t21881: F, t22544: F, t22564: F, t22565: F, t2371: F, t4158: F, t572: F, t5795: F, t5801: F, t5802: F, t5805: F, t670: F, t6945: F, t6948: F) -> (F, F, F, F) {
    let t75716 = t61014 + t75451 + t75676 + t75714;
    let t75720 = t1455 * t6951;
    let t75727 = t6936 * t1464;
    let t75760 = F::cast_from(12.0_f64) * t116 * t21881 * t572 * t670 + F::cast_from(12.0_f64) * t13514 * t572 * t5801 + F::cast_from(6.0_f64) * t22564 * t2371 * t572 + F::cast_from(12.0_f64) * t1459 * t22565 + F::cast_from(6.0_f64) * t1461 * t22544 + F::cast_from(12.0_f64) * t18204 * t1916 + F::cast_from(12.0_f64) * t18211 * t1916 + F::cast_from(6.0_f64) * t18214 * t1916 + F::cast_from(6.0_f64) * t4158 * t6945 + F::cast_from(3.0_f64) * t4158 * t6948 + F::cast_from(24.0_f64) * t5795 * t5802 + F::cast_from(12.0_f64) * t5795 * t5805;
    (t75716, t75720, t75727, t75760)
}
