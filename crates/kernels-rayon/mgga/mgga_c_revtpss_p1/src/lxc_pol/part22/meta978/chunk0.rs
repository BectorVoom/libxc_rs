//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3287/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3287(t50890: f64, t18263: f64, t2615: f64, t50892: f64, t50893: f64, t40186: f64, t40203: f64, t40205: f64, t14330: f64, t18305: f64, t2251: f64, t50901: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t62301 = 24.0_f64 * t50890;
    let t62302 = t18263 * t2615;
    let t62303 = 8.0_f64 * t62302;
    let t62304 = 2.0_f64 * t50892;
    let t62305 = 0.2077903092681775651e3_f64 * t50893;
    let t62306 = 12.0_f64 * t40186;
    let t62307 = 0.11696447245269292414e1_f64 * t40203;
    let t62308 = 0.70178683471615754484e1_f64 * t40205;
    let t62311 = 24.0_f64 * t14330 * t18305 * t2251;
    let t62312 = 0.65061487801810439052e-1_f64 * t50901;
    (t62301, t62303, t62304, t62305, t62306, t62307, t62308, t62311, t62312)
}
