//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1344/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1344(t43: f64, t55912: f64, t1492: f64, t53885: f64, t1567: f64, t870: f64, t15008: f64, t15012: f64, t15016: f64, t1579: f64, t17443: f64, t17465: f64, t17534: f64, t17543: f64, t17635: f64, t4230: f64, t4536: f64, t5098: f64, t5103: f64, t53918: f64, t8287: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t58252 = piecewise3(t44, 0.0_f64, t55912);
    let t58265 = 0.23392893589820816284e1_f64 * t53885 * t1492;
    let t58284 = t1567 * t870;
    let t58293 = 2.0_f64 / 3.0_f64 * t53918 * t1579 + t15012 * t5098 + 56.0_f64 / 27.0_f64 * t4536 * t17443 - 16.0_f64 / 3.0_f64 * t15008 * t5098 - 16.0_f64 / 9.0_f64 * t4230 * t17635 - 448.0_f64 / 81.0_f64 * t4230 * t17443 + 352.0_f64 / 27.0_f64 * t15016 * t5103 + 128.0_f64 / 9.0_f64 * t4230 * t17465 + 16000000.0_f64 / 243.0_f64 * t8287 * t58284 * t17534 * t17543 - 16.0_f64 / 3.0_f64 * t4536 * t17465 + 88.0_f64 / 9.0_f64 * t15016 * t5098;
    (t58252, t58265, t58293)
}
