//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 652/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk652<F: Float>(t5180: F, t5182: F, t1175: F, t1804: F, t375: F, t5084: F, t5087: F, t5089: F, t5092: F, t5094: F, t5097: F, t5100: F, t5166: F, t5170: F, t5173: F, t5178: F) -> (F, F, F, F) {
    let t5183 = t5180 * t5182;
    let t5185 = t1175 * t1804;
    let t5186 = t375 * t5185;
    let t5188 = -t5084 / F::new(16.0) + t5087 / F::new(36.0) + t5089 / F::new(256.0) - t5092 / F::new(24.0) + t5094 / F::new(48.0) - t5097 / F::new(48.0) + t5100 / F::new(192.0) + t5166 / F::new(16.0) - t5170 / F::new(256.0) + t5173 / F::new(256.0) - t5178 / F::new(128.0) - t5183 / F::new(576.0) - t5186 / F::new(9.0);
    (t5183, t5185, t5186, t5188)
}
