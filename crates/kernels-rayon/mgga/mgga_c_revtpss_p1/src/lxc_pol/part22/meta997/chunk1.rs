//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3388/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3388(t15474: f64, t1610: f64, t2874: f64, t11299: f64, t2918: f64, t6145: f64, t11528: f64, t19327: f64, t19128: f64, t934: f64, t6142: f64, t19330: f64, t2875: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63633 = 4.0_f64 * t2874 * t1610 * t15474;
    let t63636 = 0.96491876992155210402e2_f64 * t11299 * t6145 * t2918;
    let t63638 = 4.0_f64 * t11528 * t19327;
    let t63641 = 4.0_f64 * t2874 * t19128 * t934;
    let t63644 = 2.0_f64 * t2874 * t6142 * t2918;
    let t63647 = 0.96491876992155210402e2_f64 * t11299 * t19330 * t2875;
    (t63633, t63636, t63638, t63641, t63644, t63647)
}
