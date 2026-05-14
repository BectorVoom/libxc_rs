//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1119/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1119<F: Float>(t23842: F, t94394: F, t172: F, t549: F, t72: F, t23723: F, t1355: F, t93191: F, t93188: F, t92529: F, t2058: F, t22626: F, t542: F, t550: F, t527: F, t133: F) -> (F, F, F, F, F, F, F, F, F) {
    let t94549 = t23842 * t94394;
    let t94552 = t549 * t172 * t72;
    let t94553 = t23723 * t94552;
    let t94578 = t1355 * t93191;
    let t94582 = t1355 * t93188;
    let t94600 = 0.18521666970164609055e-1 * t1355 * t92529;
    let t94601 = t2058 * t22626;
    let t94602 = t542 * t94601;
    let t94607 = t550 * t22626;
    let t94608 = t527 * t94607;
    let t94613 = t133 * t94601;
    (t94549, t94553, t94578, t94582, t94600, t94602, t94607, t94608, t94613)
}
