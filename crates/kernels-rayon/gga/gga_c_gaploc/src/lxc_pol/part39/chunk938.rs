//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 938/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk938(t40353: f64, t12964: f64, t2487: f64, t6985: f64, t9078: f64, t986: f64, t544: f64, t2386: f64, t204: f64, t2476: f64, t41738: f64, t10615: f64, t1423: f64, t3129: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42144 = 0.11502877786176224903e1_f64 * t40353;
    let t42146 = t2487 * t6985 * t12964;
    let t42148 = t9078 * t986;
    let t42149 = t544 * t42148;
    let t42151 = 0.53625734927775640005e1_f64 * t42149 * t2386;
    let t42154 = 0.92023022289409799224e1_f64 * t2476 * t204 * t41738;
    let t42156 = t10615 * t1423 * t3129;
    (t42144, t42146, t42148, t42151, t42154, t42156)
}
