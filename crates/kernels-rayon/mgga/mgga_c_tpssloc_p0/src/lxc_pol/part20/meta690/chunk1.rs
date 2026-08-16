//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2619/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2619(t11797: f64, t5005: f64, t1174: f64, t5045: f64, t698: f64, t3540: f64, t4966: f64, t11647: f64, t1744: f64, t11825: f64, t45167: f64, t45169: f64, t45171: f64, t45178: f64, t45181: f64, t45184: f64, t4974: f64) -> f64 {
    let t53267 = t5005 * t11797;
    let t53270 = t1174 * t698 * t5045;
    let t53271 = t53270 / 432.0_f64;
    let t53272 = t4966 * t3540;
    let t53273 = t53272 / 4608.0_f64;
    let t53274 = t1744 * t11647;
    let t53276 = t45167 / 1536.0_f64 + t45169 / 768.0_f64 - t45171 / 1536.0_f64 + t45178 / 216.0_f64 - t45181 / 864.0_f64 - t45184 / 144.0_f64 - t11825 * t4974 / 768.0_f64 - t53267 / 2304.0_f64 + t53271 - t53273 - t53274 / 1944.0_f64;
    t53276
}
