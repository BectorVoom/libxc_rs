//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 711/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk711(t11204: f64, t35: f64, t534: f64, t7858: f64, t1597: f64, t1655: f64, t1594: f64, t3099: f64, t408: f64, t1685: f64, t3070: f64, t11127: f64, t11131: f64, t11136: f64, t11142: f64, t11147: f64, t11150: f64, t11155: f64, t11160: f64, t1617: f64, t1620: f64, t1624: f64, t1669: f64, t3019: f64, t3022: f64, t372: f64, t374: f64, t401: f64, t7919: f64, t7982: f64, t8000: f64, t8003: f64, t931: f64) -> f64 {
    let t11205 = t11204 * t35;
    let t11209 = t534 * t7858;
    let t11212 = t1655 * t1597;
    let t11213 = t1594 * t11212;
    let t11216 = t408 * t3099;
    let t11220 = t3070 * t1685;
    let t11223 = -0.13784064983740990796e-3_f64 * t1617 * t11127 * t1620 - 0.16340680006645994455e-5_f64 * t8000 * t11131 * t8003 + 0.13519760450715832853e-3_f64 * t3019 * t11136 - 0.19365723406274399941e-3_f64 * t1624 * t11142 + 0.19365723406274399941e-3_f64 * t372 * t11147 - 0.32253953169881963531e-5_f64 * t372 * t11150 - 0.23254900946437792e-2_f64 * t1624 * t11155 - 0.23254900946437792e-1_f64 * t7919 * t931 - 0.67598802253579164263e-4_f64 * t7982 * t11160 - 0.11627450473218896e-1_f64 * t372 * t374 * t11205 - 0.33776098467676728323e-5_f64 * t11209 * t3022 + 0.67598802253579164263e-4_f64 * t11213 * t3022 - 4.0_f64 * t1669 * t11216 * t401 - 2.0_f64 * t1669 * t11220;
    t11223
}
