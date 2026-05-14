//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1113/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1113<F: Float>(t458: F, t5494: F, t5504: F, t1285: F, t1771: F, t22873: F, t378: F, t22870: F, t5495: F, t1286: F, t1637: F, t5619: F, t22895: F, t376: F, t22864: F, t22899: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t94032 = t5494 * t458;
    let t94033 = t94032 * t5504;
    let t94035 = t1285 * t1771;
    let t94036 = t94035 * t5504;
    let t94038 = t378 * t22873;
    let t94046 = t5495 * t22870;
    let t94049 = t1286 * t1637 * t5619;
    let t94067 = t1286 * t376 * t22895;
    let t94070 = t1286 * t376 * t22864;
    let t94081 = t1286 * t376 * t22899;
    (t94032, t94033, t94035, t94036, t94038, t94046, t94049, t94067, t94070, t94081)
}
