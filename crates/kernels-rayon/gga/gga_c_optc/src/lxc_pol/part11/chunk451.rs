//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 451/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk451(t334: f64, t906: f64, t317: f64, t909: f64, t282: f64, t911: f64, t115: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t2693 = 1.0_f64 / t906 / t334;
    let t2694 = t317 * t2693;
    let t2718 = t909 * sigma0;
    let t2719 = t282 * t911;
    let t2720 = t2719 * t115;
    let t2721 = t2718 * t2720;
    (t2693, t2694, t2718, t2719, t2721)
}
