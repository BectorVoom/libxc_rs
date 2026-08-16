//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3144/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3144(t12429: f64, t1744: f64, t12423: f64, t12430: f64, t12464: f64, t12508: f64, t12511: f64, t16951: f64, t16955: f64, t16958: f64, t16962: f64, t16965: f64, t16966: f64, t1745: f64, t3452: f64, t3453: f64, t3471: f64, t45080: f64, t45085: f64, t45197: f64, t5143: f64, t56279: f64, t56281: f64, t56283: f64, t56286: f64, t56290: f64, t57799: f64) -> f64 {
    let t57944 = t12429 * t1744;
    let t57967 = -t56279 + t56281 - t56283 + t56286 - t56290 - 0.57895126195293126243e3_f64 * t57944 * t12508 - t57799 - 6.0_f64 * t12511 * t16951 - 0.57895126195293126242e3_f64 * t45197 * t16955 + 0.96491876992155210402e2_f64 * t12423 * t16962 + 0.6207121550312808036e4_f64 * t45080 * t16966 - 6.0_f64 * t3452 * t5143 * t3471 - 0.57895126195293126242e3_f64 * t12429 * t16958 * t3453 - 2.0_f64 * t3452 * t1745 * t12464 - 0.24828486201251232145e5_f64 * t45085 * t16965 * t12430;
    t57967
}
