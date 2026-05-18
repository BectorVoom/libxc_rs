//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 821/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk821<F: Float>(t1111: F, t1198: F, t1201: F, t1209: F, t1472: F, t19053: F, t19107: F, t21235: F, t21264: F, t21996: F, t22000: F, t22003: F, t22007: F, t22013: F, t22020: F, t22059: F, t2691: F, t285: F, t292: F, t4099: F, t4113: F, t4114: F, t5016: F, t5232: F, t5262: F, t5265: F, t5266: F, t5295: F, t7003: F, t817: F) -> F {
    let t22062 = F::new(12.0) * t2691 * t21996 - F::new(0.32588984563981206924e0) * t7003 * t22000 + F::new(0.72490960660845957359e1) * t19053 * t22003 - F::new(0.1812274016521148934e1) * t1472 * t22007 + F::new(0.1812274016521148934e1) * t4099 * t22007 + F::new(0.87582322958871935982e1) * t19107 * t22013 + F::new(0.3624548033042297868e1) * t5262 * t1111 + F::new(0.87582322958871935983e1) * t1198 * t5016 - F::new(6.0) * t2691 * t22020 + F::new(0.10862994854660402308e0) * t4113 * t22000 + F::new(6.0) * t4113 * t4114 * t5295 + F::new(0.3624548033042297868e1) * t5232 * t1111 - F::new(0.21895580739717983995e1) * t5265 * t5266 * t1209 + F::new(0.28056686626142231644e2) * t292 * t21235 - F::new(0.27734402270309446394e2) * t292 * t21264 - F::new(0.28056686626142231644e2) * t1201 * t21235 + F::new(0.55468804540618892788e2) * t1201 * t21264 - t285 * t817 * t22059;
    t22062
}
