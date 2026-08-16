//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1182/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1182(t1208: f64, t2035: f64, t22090: f64, t1111: f64, t1201: f64, t19039: f64, t19101: f64, t19107: f64, t19135: f64, t22135: f64, t292: f64, t4092: f64, t5016: f64, t5232: f64, t5239: f64, t5261: f64, t5265: f64, t5266: f64, t5296: f64, t70475: f64, t70476: f64, t7607: f64, t88444: f64, t90192: f64, t90224: f64) -> f64 {
    let t90239 = t2035 * t22090 * t1208;
    let t90264 = 0.17516464591774387196e2_f64 * t7607 * t90224 + 0.11093760908123778558e3_f64 * t19135 * t90239 - 0.26248964422271975727e0_f64 * t1201 * t88444 + 0.65622411055679939316e-1_f64 * t292 * t88444 + 0.87582322958871935983e1_f64 * t19039 * t5266 * t5261 + 0.17516464591774387197e2_f64 * t5232 * t5016 - 0.4832730710723063824e1_f64 * t4092 * t22135 * t1111 + 0.44375043632495114232e3_f64 * t19101 * t90192 - 0.22187521816247557116e3_f64 * t19107 * t90239 - 0.17516464591774387196e2_f64 * t70475 * t70476 * t5239 - 0.43791161479435967991e1_f64 * t5265 * t5266 * t5296;
    t90264
}
