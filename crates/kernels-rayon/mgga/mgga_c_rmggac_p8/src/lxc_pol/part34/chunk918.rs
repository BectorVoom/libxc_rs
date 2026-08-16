//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 918/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk918(t70545: f64, t14102: f64, t8365: f64, t638: f64, t639: f64, t640: f64, t9030: f64, t2046: f64, t3047: f64, t8850: f64, t8854: f64, t36292: f64, t739: f64, t8936: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76515 = 0.79828278012425390427e-1_f64 * t70545;
    let t76517 = t8365 * t14102;
    let t76521 = t638 * t639 * t640 * t9030;
    let t76524 = t2046 * t3047 * t8850;
    let t76527 = t2046 * t3047 * t8854;
    let t76538 = t739 * t36292 * t8936;
    (t76515, t76517, t76521, t76524, t76527, t76538)
}
