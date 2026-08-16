//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1436/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1436(t10292: f64, t11143: f64, t11142: f64, t856: f64, t2231: f64, t31458: f64, t31461: f64, t31463: f64, t31465: f64, t31468: f64, t31469: f64, t31470: f64, t31472: f64, t31474: f64, t31476: f64, t31478: f64, t31480: f64, t31483: f64, t31485: f64, t32090: f64, t32091: f64, t32093: f64, t32095: f64, t3513: f64) -> (f64, f64, f64) {
    let t35257 = 2.0_f64 * t10292;
    let t35259 = 2.0_f64 * t11143;
    let t39538 = t856 * t11142;
    let t39539 = t2231 * t3513 - t31458 - t31461 - t31463 + t31465 + t31468 - t31469 - t31470 + t31472 - t31474 + t31476 + t31478 + t31480 + t31483 - t31485 + t32090 - t32091 - t32093 + t32095 + t39538;
    (t35257, t35259, t39539)
}
