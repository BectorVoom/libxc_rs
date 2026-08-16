//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1045/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1045(t32244: f64, t45844: f64, t12571: f64, t116935: f64, t33107: f64, t116919: f64, t33119: f64, t32248: f64, t116909: f64, t33111: f64, t116905: f64, t116932: f64, t116942: f64, t116947: f64, t116954: f64, t119884: f64, t119892: f64, t119909: f64, t119955: f64, t119971: f64, t119975: f64, t119990: f64, t31006: f64, t31013: f64, t31024: f64, t32245: f64, t32258: f64, t8707: f64) -> f64 {
    let t124335 = t45844 * t32244;
    let t124338 = t12571 * t32244;
    let t124351 = t116935 * t33107;
    let t124353 = t116919 * t33119;
    let t124355 = t12571 * t32248;
    let t124360 = t116909 * t33111;
    let t124364 = 10.0_f64 / 3.0_f64 * t116905 * t119884 - 10.0_f64 / 9.0_f64 * t116954 * t119892 - 5.0_f64 / 6.0_f64 * t124335 * t31006 + 5.0_f64 / 18.0_f64 * t124338 * t31024 - 5.0_f64 / 6.0_f64 * t116942 * t33107 - 5.0_f64 / 6.0_f64 * t32245 * t119990 + 5.0_f64 / 18.0_f64 * t116947 * t33119 + 5.0_f64 / 18.0_f64 * t32258 * t119971 + 5.0_f64 / 18.0_f64 * t32258 * t119975 + 40.0_f64 / 9.0_f64 * t124351 - 40.0_f64 / 27.0_f64 * t124353 - 5.0_f64 / 9.0_f64 * t124355 * t31013 + 35.0_f64 / 6.0_f64 * t116932 * t119909 + 80.0_f64 / 27.0_f64 * t124360 + 5.0_f64 / 36.0_f64 * t119955 * t8707;
    t124364
}
