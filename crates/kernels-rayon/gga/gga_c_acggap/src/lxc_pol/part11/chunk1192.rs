//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1192/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1192(t36299: f64, t30159: f64, t36213: f64, t7586: f64, t2041: f64, t4777: f64, t4781: f64, t4787: f64, t2030: f64, t2288: f64, t4262: f64, t839: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36300 = 0.14291339372689912324e-2_f64 * t36299;
    let t36302 = t30159 * t7586 * t36213;
    let t36303 = 0.85748036236139473944e-3_f64 * t36302;
    let t36306 = t2041 * t4777;
    let t36308 = t2041 * t4781;
    let t36310 = t2041 * t4787;
    let t36314 = t2030 * t4262 * t2288 * t839;
    (t36300, t36303, t36306, t36308, t36310, t36314)
}
