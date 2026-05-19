//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1424/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1424<F: Float>(t35177: F, t35182: F, t35190: F, t35192: F, t37223: F, t37224: F, t37227: F, t37228: F, t37229: F, t37232: F, t37233: F, t35203: F, t37236: F, t37237: F, t37238: F, t37239: F, t37240: F, t37241: F, t37242: F, t37243: F, t37244: F, t37245: F) -> (F, F) {
    let t38667 = t37223 + t37224 - F::cast_from(0.10925861285174334492e-8_f64) * t35177 - F::cast_from(0.38527756621470067413e-7_f64) * t35182 - t37227 - t37228 + t37229 - F::cast_from(0.31433990684987949196e-7_f64) * t35190 - F::cast_from(0.67632724766374884054e-5_f64) * t35192 + t37232 - t37233;
    let t38669 = F::cast_from(0.72463633678258804344e-6_f64) * t35203 + t37236 + t37237 - t37238 - t37239 + t37240 - t37241 + t37242 + t37243 + t37244 - t37245;
    (t38667, t38669)
}
