//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1532/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1532(t1065: f64, t23598: f64, t11630: f64, t23829: f64, t3172: f64, t1011: f64, t140: f64, t24016: f64, t11710: f64, t23907: f64, t3091: f64, t23912: f64) -> (f64, f64, f64, f64, f64) {
    let t79301 = t1065 * t23598;
    let t79309 = t11630 * t3172 * t23829;
    let t79315 = t1011 * t140 * t24016;
    let t79428 = t3091 * t11710 * t23907;
    let t79439 = t3091 * t11710 * t23912;
    (t79301, t79309, t79315, t79428, t79439)
}
