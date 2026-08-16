//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 774/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk774(t30: f64, t1165: f64, t1338: f64, t3493: f64, t4631: f64, t4637: f64, t4674: f64, t93: f64, t4356: f64, t4358: f64, t1288: f64, t3282: f64, t4578: f64, t490: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t5322 = 2.0_f64 * t1165 * t4674 + 4.0_f64 * t1338 * t3493 + 2.0_f64 * t4637 * t93 + t4631;
    let t5326 = 8.0_f64 * t4356;
    let t5327 = 8.0_f64 * t4358;
    let t5328 = t1288 * t1288;
    let t5334 = piecewise3(t31, 0.0_f64, 4.0_f64 / 9.0_f64 * t3282 * t5328 + 4.0_f64 / 3.0_f64 * t490 * t4578);
    (t5322, t5326, t5327, t5328, t5334)
}
