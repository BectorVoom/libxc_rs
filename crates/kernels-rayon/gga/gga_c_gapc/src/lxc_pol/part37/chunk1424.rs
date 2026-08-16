//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1424/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1424(t35177: f64, t35182: f64, t35190: f64, t35192: f64, t37223: f64, t37224: f64, t37227: f64, t37228: f64, t37229: f64, t37232: f64, t37233: f64, t35203: f64, t37236: f64, t37237: f64, t37238: f64, t37239: f64, t37240: f64, t37241: f64, t37242: f64, t37243: f64, t37244: f64, t37245: f64) -> (f64, f64) {
    let t38667 = t37223 + t37224 - 0.10925861285174334492e-8_f64 * t35177 - 0.38527756621470067413e-7_f64 * t35182 - t37227 - t37228 + t37229 - 0.31433990684987949196e-7_f64 * t35190 - 0.67632724766374884054e-5_f64 * t35192 + t37232 - t37233;
    let t38669 = 0.72463633678258804344e-6_f64 * t35203 + t37236 + t37237 - t37238 - t37239 + t37240 - t37241 + t37242 + t37243 + t37244 - t37245;
    (t38667, t38669)
}
