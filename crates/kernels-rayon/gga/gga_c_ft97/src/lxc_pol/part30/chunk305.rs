//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 305/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk305(t292: f64, t4125: f64, t817: f64, t1472: f64, t2691: f64, t285: f64, t4062: f64, t4065: f64, t4090: f64, t4094: f64, t4096: f64, t4099: f64, t4101: f64, t4104: f64, t4110: f64, t4113: f64, t4115: f64) -> (f64, f64) {
    let t293 = 0.1e-59_f64 < t292;
    let t4126 = t817 * t4125;
    let t4129 = piecewise3(t293, 2.0_f64 * t4062 - 2.0_f64 * t2691 * t4065 + 2.0_f64 * t4090 - 0.1208182677680765956e1_f64 * t4094 * t4096 + 0.60409133884038297798e0_f64 * t4099 * t4101 + 0.1208182677680765956e1_f64 * t4104 * t4096 - 0.60409133884038297798e0_f64 * t1472 * t4101 - 2.0_f64 * t2691 * t4110 + 2.0_f64 * t4113 * t4115 - t285 * t4126, 0.0_f64);
    (t4126, t4129)
}
