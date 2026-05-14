//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 498/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk498<F: Float>(t3213: F, t3288: F, t103: F, t3170: F, t108: F, t2976: F, t3109: F, t3220: F, t3239: F, t3256: F, t3262: F, t438: F, t497: F, t88: F, t948: F, t984: F) -> (F, F, F) {
    let t3289 = t3213 + t3288;
    let t3291 = t3170 * t103;
    let t3297 = -t108 * t2976 - t108 * t3109 - t3289 * t88 - t438 * t984 - t497 * t948 + 4.0 * t3220 - 2.0 * t3239 - 2.0 * t3256 - 2.0 * t3262 + 2.0 * t3291;
    (t3289, t3291, t3297)
}
