//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 934/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk934(t1058: f64, t5165: f64, t2706: f64, t639: f64, t1535: f64, t1673: f64, t2536: f64, t4996: f64, t5005: f64, t5011: f64, t5019: f64, t5022: f64, t5148: f64, t5154: f64, t5170: f64, t568: f64, t7023: f64, t7025: f64, t7030: f64, t7031: f64, t7032: f64, t7034: f64, t7037: f64, t7039: f64, t7041: f64, t7042: f64) -> (f64, f64, f64) {
    let t7197 = t1058 * t5165;
    let t7201 = t2706 * t639;
    let t7205 = 6.0_f64 * t1535 * t568 * t7201 + 2.0_f64 * t1673 * t2536 * t7197 + t4996 + t5005 - t5011 + t5019 - t5022 - t5148 - t5154 + t5170 + t7023 + t7025 + t7030 - t7031 - t7032 - t7034 - t7037 - t7039 + t7041 - t7042;
    (t7197, t7201, t7205)
}
