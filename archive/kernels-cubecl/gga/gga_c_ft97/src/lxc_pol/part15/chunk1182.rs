//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1182/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1182<F: Float>(t1208: F, t2035: F, t22090: F, t1111: F, t1201: F, t19039: F, t19101: F, t19107: F, t19135: F, t22135: F, t292: F, t4092: F, t5016: F, t5232: F, t5239: F, t5261: F, t5265: F, t5266: F, t5296: F, t70475: F, t70476: F, t7607: F, t88444: F, t90192: F, t90224: F) -> F {
    let t90239 = t2035 * t22090 * t1208;
    let t90264 = F::cast_from(0.17516464591774387196e2_f64) * t7607 * t90224 + F::cast_from(0.11093760908123778558e3_f64) * t19135 * t90239 - F::cast_from(0.26248964422271975727e0_f64) * t1201 * t88444 + F::cast_from(0.65622411055679939316e-1_f64) * t292 * t88444 + F::cast_from(0.87582322958871935983e1_f64) * t19039 * t5266 * t5261 + F::cast_from(0.17516464591774387197e2_f64) * t5232 * t5016 - F::cast_from(0.4832730710723063824e1_f64) * t4092 * t22135 * t1111 + F::cast_from(0.44375043632495114232e3_f64) * t19101 * t90192 - F::cast_from(0.22187521816247557116e3_f64) * t19107 * t90239 - F::cast_from(0.17516464591774387196e2_f64) * t70475 * t70476 * t5239 - F::cast_from(0.43791161479435967991e1_f64) * t5265 * t5266 * t5296;
    t90264
}
