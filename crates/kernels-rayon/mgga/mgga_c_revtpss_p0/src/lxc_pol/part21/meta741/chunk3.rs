//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2610/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2610(t48039: f64, t14220: f64, t46495: f64, t4086: f64, t5710: f64, t786: f64, t4104: f64, t14255: f64, t1883: f64, t3924: f64, t46472: f64, t46490: f64, t46493: f64, t47396: f64, t48027: f64, t48029: f64, t48036: f64, t820: f64) -> f64 {
    let t48040 = 0.39029762157531132076e-1_f64 * t48039;
    let t48041 = t46495 * t14220;
    let t48042 = 0.34697458558045176417e-2_f64 * t48041;
    let t48048 = t786 * t4086 * t5710;
    let t48049 = t48048 * t4104;
    let t48052 = -0.32927245914677557992e-1_f64 * t48027 - 0.39029762157531132075e-2_f64 * t48029 + 0.32927245914677557992e-1_f64 * t46472 - 0.65854491829355115987e0_f64 * t820 * t47396 * t1883 + 0.46263278077393568556e-2_f64 * t48036 + t48040 - t48042 - 0.19756347548806534796e1_f64 * t820 * t14255 * t3924 + 0.39029762157531132075e-1_f64 * t46490 - 0.58544643236296698113e-1_f64 * t48049 - 0.32927245914677557992e-1_f64 * t46493;
    t48052
}
