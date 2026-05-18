//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 526/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk526<F: Float>(t292: F, t4125: F, t817: F, t1472: F, t2691: F, t285: F, t4062: F, t4065: F, t4090: F, t4094: F, t4096: F, t4099: F, t4101: F, t4104: F, t4110: F, t4113: F, t4115: F) -> F {
    let t293 = F::new(0.1e-59) < t292;
    let t4126 = t817 * t4125;
    let t4129 = piecewise3::<f64>(t293, F::new(2.0) * t4062 - F::new(2.0) * t2691 * t4065 + F::new(2.0) * t4090 - F::new(0.1208182677680765956e1) * t4094 * t4096 + F::new(0.60409133884038297798e0) * t4099 * t4101 + F::new(0.1208182677680765956e1) * t4104 * t4096 - F::new(0.60409133884038297798e0) * t1472 * t4101 - F::new(2.0) * t2691 * t4110 + F::new(2.0) * t4113 * t4115 - t285 * t4126, F::new(0.0));
    t4129
}
