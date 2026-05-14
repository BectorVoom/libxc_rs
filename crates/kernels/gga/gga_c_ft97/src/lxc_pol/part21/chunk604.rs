//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 604/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk604<F: Float>(t12365: F, t3379: F, t549: F, t135: F, t3347: F, t1008: F, t2057: F, t131: F, t538: F, t550: F, t1995: F, t527: F, t422: F, t929: F, t71: F, t11260: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12366 = t12365 / 9.0;
    let t12367 = t549 * t3379;
    let t12374 = t3347 * t135;
    let t12401 = t2057 * t1008;
    let t12411 = t538 * t131;
    let t12448 = t550 * t1008;
    let t12449 = t1995 * t12448;
    let t12452 = t527 * t12448;
    let t12477 = t422 * t929;
    let t12486 = t71 * t929;
    let t12527 = 0.22226000364197530866e-1 * t11260;
    (t12366, t12367, t12374, t12401, t12411, t12449, t12452, t12477, t12486, t12527)
}
