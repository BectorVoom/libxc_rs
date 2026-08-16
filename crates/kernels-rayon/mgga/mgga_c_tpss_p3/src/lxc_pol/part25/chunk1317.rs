//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1317/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1317(t17930: f64, t52613: f64, t4802: f64, t750: f64, t1364: f64, t555: f64, t63783: f64, t4578: f64, t821: f64, t1398: f64, t3724: f64, t19817: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69858 = t17930 * t52613;
    let t69863 = t4802 * t750;
    let t69864 = t17930 * t69863;
    let t69868 = t63783 * t555 * t1364;
    let t69871 = t4578 * t821;
    let t69881 = t1398 * t3724;
    let t69882 = t19817 * t69881;
    (t69858, t69863, t69864, t69868, t69871, t69881, t69882)
}
