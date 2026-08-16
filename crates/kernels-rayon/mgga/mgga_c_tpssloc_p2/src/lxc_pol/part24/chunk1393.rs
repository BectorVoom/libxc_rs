//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1393/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1393(t23482: f64, t23488: f64, t23509: f64, t23508: f64, t6721: f64, t6741: f64, t1937: f64, t23453: f64, t40: f64, t1000: f64, t10195: f64, t10250: f64, t131: f64, t1920: f64, t1933: f64, t23454: f64, t23515: f64, t23521: f64, t23548: f64, t2987: f64, t350: f64, t4509: f64, t6723: f64, t6735: f64, t6747: f64, t82802: f64, t83082: f64, t83085: f64, t83092: f64, t83098: f64, t83100: f64, t83111: f64) -> (f64, f64) {
    let t83114 = t23482 * t23488;
    let t83117 = t23509 * t23488;
    let t83120 = t6721 * t23508;
    let t83121 = t83120 * t6741;
    let t83127 = t23453 * t40 * t1937;
    let t83129 = -t83082 / 72.0_f64 - 0.30279567070605293142e-3_f64 * t83085 * t6747 - 0.21801288290835811062e-1_f64 * t23454 * t6735 + 0.24223653656484234513e-2_f64 * t6723 * t23548 + 11.0_f64 / 108.0_f64 * t83092 * t1000 - 77.0_f64 / 162.0_f64 * t82802 * t131 * t350 + 11.0_f64 / 108.0_f64 * t83098 + 0.10093189023535097714e-3_f64 * t1933 * t83100 * t1937 - t1920 * t2987 * t10250 / 48.0_f64 + t1920 * t4509 * t10195 / 72.0_f64 + 0.21801288290835811062e-1_f64 * t83111 * t6747 - 0.48447307312968469026e-2_f64 * t83114 * t6747 + 0.60559134141210586284e-3_f64 * t83117 * t23515 - 0.48447307312968469026e-2_f64 * t83121 * t23515 + 0.24223653656484234513e-2_f64 * t83121 * t23521 + 0.21801288290835811062e-1_f64 * t83127;
    (t83117, t83129)
}
