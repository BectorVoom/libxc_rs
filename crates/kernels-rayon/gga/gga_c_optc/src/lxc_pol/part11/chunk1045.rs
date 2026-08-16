//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1045/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1045(t24599: f64, t331: f64, t8124: f64, t25836: f64, t3145: f64, t9: f64, t2849: f64, t22: f64, t8950: f64, t8428: f64, t3016: f64, t375: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t25969 = 0.5224665647534064904e-2_f64 * t331 * t24599;
    let t25981 = t8124 * sigma0;
    let t25982 = t25981 * t25836;
    let t26133 = t9 * t3145;
    let t26134 = t26133 * t2849;
    let t26140 = t22 * t8950;
    let t26141 = t26140 * t8428;
    let t26193 = t3016 * t3016;
    let t26195 = t375 / t26193;
    (t25969, t25982, t26134, t26141, t26195)
}
