//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1033/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1033(t23685: f64, t346: f64, t349: f64, t8343: f64, t23682: f64, t2471: f64, t2475: f64, t214: f64, t211: f64, t217: f64, t22502: f64, t2528: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23686 = 0.20068888888888888889e-1_f64 * t23685;
    let t23708 = t346 / t8343 / t349;
    let t23769 = 0.75383950617283950617e4_f64 * t23682;
    let t23770 = 0.12819753086419753086e4_f64 * t23685;
    let t23800 = t2471 * t2471;
    let t23801 = 1.0_f64 / t23800;
    let t23803 = t2475 * t2475;
    let t23804 = 1.0_f64 / t23803;
    let t23844 = f64::powf(t214, -0.25e1_f64);
    let t23860 = 280.0_f64 / 81.0_f64 * t23682;
    let t23913 = 1.0_f64 / t217 / t22502 / t211 / 96.0_f64;
    let t23926 = 0.31310740740740740741e1_f64 * t23682;
    let t23927 = 0.13490888888888888889e1_f64 * t23685;
    let t24021 = 1.0_f64 / t2471 / t2528;
    (t23686, t23708, t23769, t23770, t23801, t23804, t23844, t23860, t23913, t23926, t23927, t24021)
}
