//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 927/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk927<F: Float>(t1882: F, t8551: F, t8392: F, t8496: F, t8491: F, t8536: F, t8378: F, t1855: F, t8232: F, t1913: F, t38953: F, t100: F, t38463: F) -> (F, F, F, F, F, F, F, F) {
    let t39194 = t1882 * t8551;
    let t39196 = t8392 * t8496;
    let t39198 = t8392 * t8491;
    let t39200 = t8392 * t8536;
    let t39202 = t8392 * t8378;
    let t39207 = t8232 * t1855;
    let t39228 = t38953 * t1913;
    let t39230 = t38463 * t100;
    (t39194, t39196, t39198, t39200, t39202, t39207, t39228, t39230)
}
