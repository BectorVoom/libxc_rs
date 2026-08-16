//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1241/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1241(t2937: f64, t513: f64, t506: f64, t3706: f64, t495: f64, t1165: f64, t1173: f64, t1181: f64, t1532: f64, t1552: f64, t17388: f64, t17390: f64, t17392: f64, t17395: f64, t17397: f64, t17399: f64, t17404: f64, t17409: f64, t1894: f64, t3196: f64, t3396: f64, t4450: f64, t5012: f64, t943: f64) -> f64 {
    let t22721 = t2937 * t513;
    let t22731 = t2937 * t506;
    let t22737 = t3706 * t495;
    let t22750 = 0.51448821741683684367e-2_f64 * t4450 * t1165 * t1552 * t22721 * t943 + 0.34299214494455789578e-2_f64 * t1173 * t1181 * t1894 * t3196 - 0.51448821741683684367e-2_f64 * t4450 * t1181 * t1532 * t22731 * t943 + 0.41159057393346947492e-1_f64 * t3396 * t1165 * t22737 * t5012 + 0.17149607247227894789e-2_f64 * t17388 + 0.17149607247227894789e-2_f64 * t17390 - 0.18140473443734395377e0_f64 * t17392 + 0.18140473443734395377e0_f64 * t17395 - 0.16006300097412701803e0_f64 * t17397 - 0.80031500487063509016e-1_f64 * t17399 + 0.34299214494455789578e-2_f64 * t17404 + 0.17149607247227894789e-2_f64 * t17409;
    t22750
}
