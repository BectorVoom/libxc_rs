//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1205/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1205(t1016: f64, t21438: f64, t10295: f64, t17288: f64, t17277: f64, t3366: f64, t1676: f64, t31458: f64, t31461: f64, t31463: f64, t31465: f64, t31468: f64, t31469: f64, t31470: f64, t31472: f64, t31474: f64, t31476: f64, t31478: f64, t31480: f64, t31483: f64, t31485: f64, t32090: f64, t3513: f64) -> (f64, f64, f64, f64) {
    let t32091 = t21438 * t1016;
    let t32093 = 12.0_f64 * t17288 * t10295;
    let t32095 = 2.0_f64 * t17277 * t3366;
    let t32097 = t1676 * t3513 - t31458 - t31461 - t31463 + t31465 + t31468 - t31469 - t31470 + t31472 - t31474 + t31476 + t31478 + t31480 + t31483 - t31485 + t32090 - t32091 - t32093 + t32095;
    (t32091, t32093, t32095, t32097)
}
