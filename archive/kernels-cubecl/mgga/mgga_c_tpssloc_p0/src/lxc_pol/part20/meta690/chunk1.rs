//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2619/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2619<F: Float>(t11797: F, t5005: F, t1174: F, t5045: F, t698: F, t3540: F, t4966: F, t11647: F, t1744: F, t11825: F, t45167: F, t45169: F, t45171: F, t45178: F, t45181: F, t45184: F, t4974: F) -> F {
    let t53267 = t5005 * t11797;
    let t53270 = t1174 * t698 * t5045;
    let t53271 = t53270 / F::cast_from(432.0_f64);
    let t53272 = t4966 * t3540;
    let t53273 = t53272 / F::cast_from(4608.0_f64);
    let t53274 = t1744 * t11647;
    let t53276 = t45167 / F::cast_from(1536.0_f64) + t45169 / F::cast_from(768.0_f64) - t45171 / F::cast_from(1536.0_f64) + t45178 / F::cast_from(216.0_f64) - t45181 / F::cast_from(864.0_f64) - t45184 / F::cast_from(144.0_f64) - t11825 * t4974 / F::cast_from(768.0_f64) - t53267 / F::cast_from(2304.0_f64) + t53271 - t53273 - t53274 / F::cast_from(1944.0_f64);
    t53276
}
