//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 773/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk773(t10473: f64, t3414: f64, t7495: f64, t5218: f64, t3406: f64, t7106: f64, t5211: f64, t10486: f64, t10511: f64, t7421: f64, t7460: f64, t1006: f64, t3456: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12595 = 8.0_f64 / 27.0_f64 * t10473;
    let t12596 = t7495 * t3414;
    let t12598 = 16.0_f64 / 15.0_f64 * t5218 * t12596;
    let t12599 = t7106 * t3406;
    let t12601 = 16.0_f64 / 15.0_f64 * t5211 * t12599;
    let t12602 = 8.0_f64 / 15.0_f64 * t10486;
    let t12603 = 32.0_f64 / 45.0_f64 * t10511;
    let t12604 = 4.0_f64 / 45.0_f64 * t7421;
    let t12605 = 8.0_f64 / 135.0_f64 * t7460;
    let t12607 = 4.0_f64 / 5.0_f64 * t1006 * t3456;
    (t12595, t12596, t12598, t12599, t12601, t12602, t12603, t12604, t12605, t12607)
}
