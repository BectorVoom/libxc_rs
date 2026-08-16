//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1011/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1011(t22592: f64, t8607: f64, t31759: f64, t6876: f64, t22573: f64, t8606: f64, t22575: f64, t31526: f64, t22574: f64, t31299: f64, t32193: f64, t22480: f64, t7042: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115922 = 6.0_f64 * t8607 * t22592;
    let t115924 = 6.0_f64 * t6876 * t31759;
    let t115925 = t8606 * t22573;
    let t115927 = 6.0_f64 * t115925 * t22575;
    let t115929 = 2.0_f64 * t6876 * t31526;
    let t115942 = 6.0_f64 * t22574 * t32193 * t31299;
    let t115946 = 2.0_f64 * t7042 * t22480;
    (t115922, t115924, t115927, t115929, t115942, t115946)
}
