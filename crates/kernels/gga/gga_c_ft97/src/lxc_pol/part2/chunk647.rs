//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 647/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk647<F: Float>(t398: F, t401: F, t51: F, t6: F, t11356: F, t64: F, t11247: F, t11332: F, t11335: F, t11340: F, t11344: F, t11348: F, t11353: F, t11357: F, t11361: F, t11368: F, t11372: F, t11377: F, t11380: F, t1603: F, t1624: F, t1625: F, t1626: F, t1669: F, t1698: F, t3019: F, t3077: F, t7854: F, t8042: F, t940: F) -> (F,) {
    let t11383 = t401 * t6 * t51 * t398;
    let t11386 = t64 * t11356;
    let t11389 = -0.23254900946437792e-1 * t8042 * t11332 + 8.0 * t1669 * t3077 * t11335 + 0.23254900946437792e-1 * t1624 * t11340 - 0.23254900946437792e-1 * t1603 * t11344 + 0.23254900946437792e-1 * t1624 * t11348 + 0.11627450473218896e-1 * t1624 * t11353 - 0.46509801892875584e-1 * t11357 * t1626 - 0.38731446812548799882e-3 * t11361 * t11247 * t1625 - 0.14053536537767171586e-3 * t940 * t1698 + 0.11627450473218896e-1 * t1624 * t11368 + 0.27039520901431665706e-3 * t3019 * t11372 - 0.13519760450715832853e-3 * t3019 * t11377 - 0.2370952259137005195e-1 * t11380 * t11383 + 0.2370952259137005195e-1 * t11386 * t7854;
    (t11389,)
}
