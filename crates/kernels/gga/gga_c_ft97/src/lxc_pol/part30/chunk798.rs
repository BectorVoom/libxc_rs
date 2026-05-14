//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 798/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk798<F: Float>(t34191: F, t34193: F, t34195: F, t34241: F, t36242: F, t36246: F, t36250: F, t36253: F, t36257: F, t36261: F, t36264: F, t36268: F, t446: F, t36160: F, t36202: F, t36239: F) -> (F,) {
    let t36271 = -2.0 / 3.0 * t446 * t36242 + 4.0 / 3.0 * t446 * t36246 + 2.0 / 3.0 * t446 * t36250 + t34191 + t34193 - t34195 - t446 * t36253 / 3.0 - t446 * t36257 / 9.0 + 2.0 / 3.0 * t446 * t36261 - 2.0 / 3.0 * t446 * t36264 + 2.0 / 3.0 * t446 * t36268 - t34241;
    let t36273 = t36160 + t36202 + t36239 + t36271;
    (t36273,)
}
