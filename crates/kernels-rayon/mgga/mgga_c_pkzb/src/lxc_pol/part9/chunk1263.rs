//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1263/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1263(t3052: f64, t6158: f64, t2209: f64, t836: f64, t7996: f64, t7999: f64, t18427: f64, t18430: f64, t18433: f64, t18440: f64, t18443: f64, t18445: f64, t18448: f64, t22190: f64, t22193: f64, t22196: f64, t22199: f64, t22202: f64, t22205: f64, t22207: f64, t22209: f64, t22215: f64) -> (f64, f64, f64, f64) {
    let t22217 = t3052 * t6158;
    let t22219 = t2209 * t836;
    let t22220 = t7996 * t22219;
    let t22222 = t7999 * t22219;
    let t22225 = 0.58258125e1_f64 * t22190 - 0.1237865625e0_f64 * t22193 - 0.485484375e1_f64 * t22196 + 0.6189328125e-1_f64 * t22199 - 0.3883875e1_f64 * t22202 + 0.247573125e0_f64 * t22205 - 0.3883875e1_f64 * t22207 - 0.1294625e1_f64 * t22209 + t18440 - 0.28179666666666666667e1_f64 * t18427 + 0.12077e1_f64 * t18430 - 0.301925e0_f64 * t18433 + t18443 + 0.82785e0_f64 * t18448 + 0.247573125e0_f64 * t22215 + 0.82524375e-1_f64 * t22217 + 0.58258125e1_f64 * t22220 - 0.1237865625e0_f64 * t22222 - 0.22076e1_f64 * t18445;
    (t22217, t22220, t22222, t22225)
}
