//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 459/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk459<F: Float>(t140: F, t2036: F, t543: F, t5821: F, t5824: F, t7183: F, t7191: F, t7196: F, t7207: F, t7319: F, t7335: F) -> (F,) {
    let t141 = 0.1e-59 < t140;
    let t7339 = piecewise3(t141, 0.10263553471742804997e0 * t2036 * t7319 - 0.41054213886971219988e0 * t543 * t7191 - 0.90629106640255751116e-1 * t5821 * t7196 + 0.22653425206514361674e0 * t543 * t7183 + 0.20527106943485609994e0 * t140 * t7191 + 0.90629106640255751116e-1 * t5824 * t7196 - 0.22653425206514361674e0 * t140 * t7183 + 0.40013602467334010748e-1 * t7335 * t7207, 0.0);
    (t7339,)
}
