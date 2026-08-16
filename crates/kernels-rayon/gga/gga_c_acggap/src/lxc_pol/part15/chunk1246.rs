//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1246/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1246(t31259: f64, t31297: f64, t31318: f64, t32760: f64, t32763: f64, t32765: f64, t35506: f64, t35507: f64, t35508: f64, t35527: f64, t35529: f64, t35538: f64, t35539: f64, t35541: f64, t37583: f64, t37584: f64, t40029: f64, t40034: f64) -> f64 {
    let t41903 = 0.39221874999999999999e0_f64 * t31259 + t35506 - t35507 - t35508 + t37583 + t37584 - t32760 - 0.20579528696673473747e-1_f64 * t40029 - t35527 + t32763 - 0.13719685797782315831e-1_f64 * t35529 - t32765 - 0.31448092289604152069e-2_f64 * t31297 - 0.42874018118069736972e-2_f64 * t40034 - t35538 + t35539 + 0.11321313224257494745e-1_f64 * t31318 + t35541;
    t41903
}
