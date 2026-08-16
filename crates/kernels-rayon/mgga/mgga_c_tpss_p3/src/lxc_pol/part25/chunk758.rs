//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 758/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk758(t2862: f64, t5082: f64, t1509: f64, t2868: f64, t2872: f64, t4044: f64, t5066: f64, t5070: f64, t5074: f64, t1025: f64, t2885: f64, t1032: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5084 = 2.0_f64 * t2862 * t5082;
    let t5085 = t1509 * t1509;
    let t5086 = t2868 * t5085;
    let t5092 = t2872 - 2.0_f64 / 9.0_f64 * t4044 - 2.0_f64 / 9.0_f64 * t5066 + 2.0_f64 / 3.0_f64 * t5070 + t5074 / 3.0_f64;
    let t5093 = t1025 * t5092;
    let t5099 = t2885 * t5085;
    let t5101 = t1032 * t5092;
    (t5084, t5085, t5086, t5092, t5093, t5099, t5101)
}
