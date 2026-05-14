//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1009/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1009<F: Float>(t16891: F, t5784: F, t1008: F, t131: F, t4466: F, t71: F, t135: F, t1060: F, t2101: F, t9114: F, t9132: F, t1045: F, t526: F, t4790: F, t582: F, t12664: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t61654 = t16891 * t5784;
    let t61671 = t1008 * t131;
    let t61672 = t61671 * t5784;
    let t61889 = t71 * t4466;
    let t62087 = t16891 * t135;
    let t62981 = t2101 * t1060;
    let t62985 = t9114 * t1060;
    let t63052 = t9132 * t1060;
    let t63180 = t526 * t1045;
    let t63258 = t582 * t4790;
    let t63304 = t582 * t12664;
    (t61654, t61672, t61889, t62087, t62981, t62985, t63052, t63180, t63258, t63304)
}
