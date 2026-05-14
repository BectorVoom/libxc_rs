//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 754/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk754<F: Float>(t7335: F, t7386: F, t7389: F, t2793: F, t694: F, t2826: F, t713: F, t1070: F, t1854: F, t1088: F, t1915: F, t2743: F, t663: F, t1100: F, t1954: F, t2848: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7420 = 0.59793333333333333334e0 * t7335;
    let t7434 = 0.32862666666666666666e0 * t7386;
    let t7435 = 0.32862666666666666666e0 * t7389;
    let t7447 = t2793 * t694;
    let t7451 = 0.60385e0 * t7335;
    let t7465 = 0.33114e0 * t7386;
    let t7466 = 0.33114e0 * t7389;
    let t7478 = t2826 * t713;
    let t7483 = t1070 * t1854;
    let t7486 = t1088 * t1915;
    let t7489 = t2743 * t663;
    let t7494 = t1100 * t1954;
    let t7500 = 0.35616666666666666666e-1 * t7335;
    let t7508 = 0.18541666666666666667e-1 * t7335;
    let t7516 = 0.34246666666666666666e-1 * t7335;
    let t7527 = t1954 * t2848;
    (t7420, t7434, t7435, t7447, t7451, t7465, t7466, t7478, t7483, t7486, t7489, t7494, t7500, t7508, t7516, t7527)
}
