//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1080/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1080(t1352: f64, t3850: f64, t1388: f64, t3914: f64, t1307: f64, t3698: f64, t3719: f64, t1395: f64, t2319: f64, t12020: f64, t225: f64, t12022: f64, t1985: f64, t6889: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55003 = t1352 * t3850;
    let t55173 = t3914 * t1388;
    let t55183 = t3698 * t1307;
    let t55246 = t1388 * t3719;
    let t55344 = t1395 * t2319;
    let t80640 = t225 * t12020;
    let t80643 = t1985 * t6889 * t80640 * t12022;
    (t55003, t55173, t55183, t55246, t55344, t80643)
}
