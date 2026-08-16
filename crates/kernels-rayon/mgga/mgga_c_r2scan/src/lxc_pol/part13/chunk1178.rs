//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1178/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1178(t10823: f64, t7601: f64, t11640: f64, t24039: f64, t11643: f64, t22731: f64, t11654: f64, t6395: f64, t10869: f64, t10811: f64, t2651: f64, t10903: f64, t11764: f64, t2207: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40145 = t7601 * t10823;
    let t40149 = t24039 * t11640;
    let t40151 = t22731 * t11643;
    let t40153 = t6395 * t11654;
    let t40155 = t7601 * t10869;
    let t40156 = 0.46574606203128791246e-1_f64 * t40155;
    let t40157 = t2651 * t10811;
    let t40158 = 0.23115257973478049502e0_f64 * t40157;
    let t40162 = t2207 * t10903 * t11764;
    (t40145, t40149, t40151, t40153, t40156, t40158, t40162)
}
