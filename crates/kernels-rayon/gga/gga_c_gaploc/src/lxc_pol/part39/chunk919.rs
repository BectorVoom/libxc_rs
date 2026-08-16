//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 919/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk919(t2859: f64, t3137: f64, t4752: f64, t10557: f64, t9431: f64, t2487: f64, t41749: f64, t6711: f64, t41810: f64, t6710: f64, t3338: f64, t874: f64) -> (f64, f64, f64, f64, f64) {
    let t41829 = 0.7150097990370085334e0_f64 * t2859 * t4752 * t3137;
    let t41831 = 0.42900587942220512003e1_f64 * t10557 * t9431;
    let t41834 = 0.87421871174939309262e2_f64 * t2487 * t6711 * t41749;
    let t41837 = 0.11502877786176224903e2_f64 * t6710 * t6711 * t41810;
    let t41838 = t3338 * t874;
    (t41829, t41831, t41834, t41837, t41838)
}
