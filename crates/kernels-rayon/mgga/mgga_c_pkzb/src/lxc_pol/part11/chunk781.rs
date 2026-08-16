//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 781/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk781(t7335: f64, t7386: f64, t7389: f64, t2793: f64, t694: f64, t2826: f64, t713: f64, t1070: f64, t1854: f64, t1088: f64, t1915: f64, t2743: f64, t663: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7420 = 0.59793333333333333334e0_f64 * t7335;
    let t7434 = 0.32862666666666666666e0_f64 * t7386;
    let t7435 = 0.32862666666666666666e0_f64 * t7389;
    let t7447 = t2793 * t694;
    let t7451 = 0.60385e0_f64 * t7335;
    let t7465 = 0.33114e0_f64 * t7386;
    let t7466 = 0.33114e0_f64 * t7389;
    let t7478 = t2826 * t713;
    let t7483 = t1070 * t1854;
    let t7486 = t1088 * t1915;
    let t7489 = t2743 * t663;
    (t7420, t7434, t7435, t7447, t7451, t7465, t7466, t7478, t7483, t7486, t7489)
}
