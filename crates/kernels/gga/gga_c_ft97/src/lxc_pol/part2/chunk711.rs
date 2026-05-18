//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 711/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk711<F: Float>(t11204: F, t35: F, t534: F, t7858: F, t1597: F, t1655: F, t1594: F, t3099: F, t408: F, t1685: F, t3070: F, t11127: F, t11131: F, t11136: F, t11142: F, t11147: F, t11150: F, t11155: F, t11160: F, t1617: F, t1620: F, t1624: F, t1669: F, t3019: F, t3022: F, t372: F, t374: F, t401: F, t7919: F, t7982: F, t8000: F, t8003: F, t931: F) -> F {
    let t11205 = t11204 * t35;
    let t11209 = t534 * t7858;
    let t11212 = t1655 * t1597;
    let t11213 = t1594 * t11212;
    let t11216 = t408 * t3099;
    let t11220 = t3070 * t1685;
    let t11223 = -F::new(0.13784064983740990796e-3) * t1617 * t11127 * t1620 - F::new(0.16340680006645994455e-5) * t8000 * t11131 * t8003 + F::new(0.13519760450715832853e-3) * t3019 * t11136 - F::new(0.19365723406274399941e-3) * t1624 * t11142 + F::new(0.19365723406274399941e-3) * t372 * t11147 - F::new(0.32253953169881963531e-5) * t372 * t11150 - F::new(0.23254900946437792e-2) * t1624 * t11155 - F::new(0.23254900946437792e-1) * t7919 * t931 - F::new(0.67598802253579164263e-4) * t7982 * t11160 - F::new(0.11627450473218896e-1) * t372 * t374 * t11205 - F::new(0.33776098467676728323e-5) * t11209 * t3022 + F::new(0.67598802253579164263e-4) * t11213 * t3022 - F::new(4.0) * t1669 * t11216 * t401 - F::new(2.0) * t1669 * t11220;
    t11223
}
