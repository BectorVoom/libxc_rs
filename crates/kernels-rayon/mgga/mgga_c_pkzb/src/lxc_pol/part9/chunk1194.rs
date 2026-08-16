//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1194/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1194(t1878: f64, t218: f64, t2774: f64, t2778: f64, t1079: f64, t5555: f64, t17351: f64, t17354: f64, t17357: f64, t17403: f64, t17405: f64, t17408: f64, t17411: f64, t17414: f64, t17417: f64, t17454: f64, t20705: f64, t20708: f64, t20710: f64, t20717: f64, t20719: f64, t20745: f64) -> (f64, f64, f64, f64) {
    let t20748 = t218 * t1878 * t2774;
    let t20749 = 0.82785e0_f64 * t20748;
    let t20751 = t218 * t1878 * t2778;
    let t20752 = 0.82785e0_f64 * t20751;
    let t20754 = t218 * t5555 * t1079;
    let t20757 = -0.22076e1_f64 * t17405 + 0.82785e0_f64 * t17411 - 0.49671e0_f64 * t17414 - 0.16557e0_f64 * t17417 - 0.93932222222222222223e0_f64 * t20705 + 0.58258125e1_f64 * t20708 - 0.1237865625e0_f64 * t20710 + t17454 - 0.28179666666666666667e1_f64 * t17351 + 0.12077e1_f64 * t17354 - 0.301925e0_f64 * t17357 + t20717 - 0.905775e0_f64 * t20719 + 0.905775e0_f64 * t20745 + t20749 + t20752 - 0.73586666666666666667e0_f64 * t20754 + t17403 + 0.82785e0_f64 * t17408;
    (t20748, t20751, t20754, t20757)
}
