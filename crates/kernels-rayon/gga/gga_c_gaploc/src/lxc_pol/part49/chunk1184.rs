//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1184/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1184(t40192: f64, t40196: f64, t12054: f64, t9333: f64, t12065: f64, t2437: f64, t41853: f64, t41854: f64, t41863: f64, t41867: f64, t41871: f64, t41874: f64, t41876: f64, t41880: f64) -> f64 {
    let t47925 = 0.38342925953920749677e0_f64 * t40192;
    let t47926 = 0.85206502119823888171e-1_f64 * t40196;
    let t47927 = t12054 * t9333;
    let t47934 = t2437 * t12065;
    let t47936 = t41853 - t41854 - t47925 + t47926 - 0.10725146985555128001e1_f64 * t47927 + 0.11502877786176224903e2_f64 * t41863 + 0.11502877786176224903e2_f64 * t41867 + 0.11502877786176224903e2_f64 * t41871 + t41874 + 0.69017266717057349418e1_f64 * t41876 - 0.21450293971110256001e1_f64 * t41880 + 0.35750489951850426669e0_f64 * t47934;
    t47936
}
