//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 821/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk821(t1111: f64, t1198: f64, t1201: f64, t1209: f64, t1472: f64, t19053: f64, t19107: f64, t21235: f64, t21264: f64, t21996: f64, t22000: f64, t22003: f64, t22007: f64, t22013: f64, t22020: f64, t22059: f64, t2691: f64, t285: f64, t292: f64, t4099: f64, t4113: f64, t4114: f64, t5016: f64, t5232: f64, t5262: f64, t5265: f64, t5266: f64, t5295: f64, t7003: f64, t817: f64) -> f64 {
    let t22062 = 12.0_f64 * t2691 * t21996 - 0.32588984563981206924e0_f64 * t7003 * t22000 + 0.72490960660845957359e1_f64 * t19053 * t22003 - 0.1812274016521148934e1_f64 * t1472 * t22007 + 0.1812274016521148934e1_f64 * t4099 * t22007 + 0.87582322958871935982e1_f64 * t19107 * t22013 + 0.3624548033042297868e1_f64 * t5262 * t1111 + 0.87582322958871935983e1_f64 * t1198 * t5016 - 6.0_f64 * t2691 * t22020 + 0.10862994854660402308e0_f64 * t4113 * t22000 + 6.0_f64 * t4113 * t4114 * t5295 + 0.3624548033042297868e1_f64 * t5232 * t1111 - 0.21895580739717983995e1_f64 * t5265 * t5266 * t1209 + 0.28056686626142231644e2_f64 * t292 * t21235 - 0.27734402270309446394e2_f64 * t292 * t21264 - 0.28056686626142231644e2_f64 * t1201 * t21235 + 0.55468804540618892788e2_f64 * t1201 * t21264 - t285 * t817 * t22059;
    t22062
}
