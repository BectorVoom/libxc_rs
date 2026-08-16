//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1032/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1032(t36299: f64, t30159: f64, t36213: f64, t7586: f64, t2299: f64, t7630: f64, t1413: f64, t7712: f64, t2310: f64, t30248: f64, t542: f64, t1967: f64, t8855: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36300 = 0.14291339372689912324e-2_f64 * t36299;
    let t36302 = t30159 * t7586 * t36213;
    let t36303 = 0.85748036236139473944e-3_f64 * t36302;
    let t36327 = t7630 * t2299;
    let t36331 = t7712 * t1413;
    let t36332 = 0.85748036236139473944e-3_f64 * t36331;
    let t36333 = t7630 * t2310;
    let t36349 = t30248 * t542;
    let t36351 = t1967 * t8855;
    (t36300, t36303, t36327, t36332, t36333, t36349, t36351)
}
