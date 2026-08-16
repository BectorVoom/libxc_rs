//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1210/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1210(t34009: f64, t34011: f64, t34023: f64, t34029: f64, t34033: f64, t34039: f64, t36893: f64, t36900: f64, t36908: f64, t36910: f64, t36912: f64, t36913: f64, t38909: f64, t38912: f64, t38914: f64, t38916: f64, t38920: f64, t38925: f64) -> f64 {
    let t41404 = t36893 + 0.18868855373762491241e-2_f64 * t38909 + 0.85748036236139473944e-3_f64 * t34009 - 0.16772315887788881104e-1_f64 * t34011 + t36900 - 0.17149607247227894789e-2_f64 * t38912 + 0.56606566121287473725e-1_f64 * t34023 + t36908 + 0.25724410870841842183e-2_f64 * t34029 - t36910 - 0.42874018118069736972e-3_f64 * t34033 - t36912 - t36913 - 0.57165357490759649296e-3_f64 * t34039 - 0.10718504529517434243e-2_f64 * t38914 + 0.40015750243531754507e-2_f64 * t38916 - 0.10718504529517434243e-2_f64 * t38920 - 0.10718504529517434243e-2_f64 * t38925;
    t41404
}
