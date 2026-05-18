//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 539/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk539<F: Float>(t139: F, t3379: F, t527: F, t1013: F, t549: F, t538: F, t1014: F, t542: F, t133: F, t135: F, t2057: F, t554: F) -> (F, F, F, F, F, F, F, F) {
    let t3380 = t139 * t3379;
    let t3381 = t527 * t3380;
    let t3383 = t549 * t1013;
    let t3384 = t3383 * t538;
    let t3387 = t542 * t1014;
    let t3392 = t133 * t135;
    let t3393 = t2057 * t1013;
    let t3394 = t3393 * t554;
    (t3380, t3381, t3383, t3384, t3387, t3392, t3393, t3394)
}
