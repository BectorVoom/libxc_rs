//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 657/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk657(t1519: f64, t2857: f64, t1042: f64, t2862: f64, t1509: f64, t2868: f64, t1027: f64, t2836: f64, t2872: f64, t4044: f64, t4049: f64, t4054: f64, t4058: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4067 = 1.0_f64 * t2857 * t1519;
    let t4068 = t1519 * t1042;
    let t4070 = 2.0_f64 * t2862 * t4068;
    let t4071 = t2868 * t1509;
    let t4072 = t4071 * t1027;
    let t4079 = t2872 - t2836 / 9.0_f64 - t4044 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t4049 + 2.0_f64 / 3.0_f64 * t4054 + t4058 / 3.0_f64;
    (t4067, t4068, t4070, t4071, t4072, t4079)
}
