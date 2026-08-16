//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1217/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1217(t1101: f64, t1181: f64, t1894: f64, t3361: f64, t1165: f64, t21118: f64, t6138: f64, t3409: f64, t5869: f64, t1090: f64, t17085: f64, t17088: f64, t17090: f64, t17092: f64, t17105: f64, t17107: f64, t17109: f64, t1899: f64, t3396: f64, t3403: f64, t4919: f64, t5862: f64) -> f64 {
    let t22220 = t3361 * t1181 * t1894 * t1101;
    let t22236 = t3361 * t1165 * t6138 * t21118;
    let t22238 = t3409 * t5869;
    let t22243 = 0.34299214494455789578e-2_f64 * t22220 + 0.68598428988911579156e-2_f64 * t3396 * t1181 * t1899 * t1090 - 0.42874018118069736972e-2_f64 * t3403 * t1165 * t5862 * t4919 + 0.85748036236139473944e-3_f64 * t17085 + 0.12004725073059526352e-1_f64 * t17088 + 0.68598428988911579156e-2_f64 * t17090 + 0.85748036236139473945e-2_f64 * t17092 + 0.20579528696673473746e-1_f64 * t22236 - 0.40015750243531754508e-2_f64 * t22238 - 0.45351183609335988443e-1_f64 * t17105 + 0.45351183609335988443e-1_f64 * t17107 - 0.22675591804667994222e-1_f64 * t17109;
    t22243
}
