//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1322/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1322(t23388: f64, t1238: f64, t179: f64, t19193: f64, t19196: f64, t19206: f64, t22260: f64, t23367: f64, t23375: f64, t23382: f64, t23383: f64, t404: f64, t6369: f64, t6395: f64, t8319: f64, t932: f64) -> f64 {
    let t23389 = 0.14291339372689912324e-3_f64 * t23388;
    let t23390 = t23367 + 0.57165357490759649295e-3_f64 * t19193 - 0.85748036236139473944e-3_f64 * t19196 - 0.20579528696673473746e-1_f64 * t8319 * t6369 - 0.34299214494455789578e-2_f64 * t19206 - 0.85748036236139473944e-3_f64 * t23375 - 0.42874018118069736972e-3_f64 * t404 * t179 * t932 * t22260 - t23382 + 0.45732285992607719436e-2_f64 * t23383 + 0.22866142996303859718e-2_f64 * t1238 * t6395 - t23389;
    t23390
}
