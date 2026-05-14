//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 672/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk672<F: Float>(t251: F, t388: F, t4818: F, t5175: F, t3436: F, t380: F, t3346: F, t4813: F, t1175: F, t1804: F, t375: F, t5084: F, t5087: F, t5089: F, t5092: F, t5094: F, t5097: F, t5100: F, t5166: F, t5170: F, t5173: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5176 = t251 * t388;
    let t5177 = t5176 * t4818;
    let t5178 = t5175 * t5177;
    let t5180 = t380 * t3436;
    let t5181 = t251 * t3346;
    let t5182 = t5181 * t4813;
    let t5183 = t5180 * t5182;
    let t5185 = t1175 * t1804;
    let t5186 = t375 * t5185;
    let t5188 = -t5084 / 16.0 + t5087 / 36.0 + t5089 / 256.0 - t5092 / 24.0 + t5094 / 48.0 - t5097 / 48.0 + t5100 / 192.0 + t5166 / 16.0 - t5170 / 256.0 + t5173 / 256.0 - t5178 / 128.0 - t5183 / 576.0 - t5186 / 9.0;
    (t5176, t5177, t5178, t5180, t5181, t5182, t5183, t5185, t5186, t5188)
}
