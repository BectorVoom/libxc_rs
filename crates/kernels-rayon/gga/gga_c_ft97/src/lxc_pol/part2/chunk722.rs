//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 722/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk722(t11356: f64, t64: f64, t11247: f64, t11332: f64, t11335: f64, t11340: f64, t11344: f64, t11348: f64, t11353: f64, t11357: f64, t11361: f64, t11368: f64, t11372: f64, t11377: f64, t11380: f64, t11383: f64, t1603: f64, t1624: f64, t1625: f64, t1626: f64, t1669: f64, t1698: f64, t3019: f64, t3077: f64, t7854: f64, t8042: f64, t940: f64) -> f64 {
    let t11386 = t64 * t11356;
    let t11389 = -0.23254900946437792e-1_f64 * t8042 * t11332 + 8.0_f64 * t1669 * t3077 * t11335 + 0.23254900946437792e-1_f64 * t1624 * t11340 - 0.23254900946437792e-1_f64 * t1603 * t11344 + 0.23254900946437792e-1_f64 * t1624 * t11348 + 0.11627450473218896e-1_f64 * t1624 * t11353 - 0.46509801892875584e-1_f64 * t11357 * t1626 - 0.38731446812548799882e-3_f64 * t11361 * t11247 * t1625 - 0.14053536537767171586e-3_f64 * t940 * t1698 + 0.11627450473218896e-1_f64 * t1624 * t11368 + 0.27039520901431665706e-3_f64 * t3019 * t11372 - 0.13519760450715832853e-3_f64 * t3019 * t11377 - 0.2370952259137005195e-1_f64 * t11380 * t11383 + 0.2370952259137005195e-1_f64 * t11386 * t7854;
    t11389
}
