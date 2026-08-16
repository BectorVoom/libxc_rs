//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1382/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1382(t389: f64, t58614: f64, t58626: f64, t11894: f64, t15064: f64, t4281: f64, t4289: f64, t4297: f64, t43649: f64, t5229: f64, t53390: f64, t53432: f64, t53443: f64, t53445: f64, t58322: f64, t58581: f64, t58585: f64, t58591: f64, t58596: f64) -> (f64, f64) {
    let t58629 = 0.62182e-1_f64 * (t58614 + t58626) * t389;
    let t58633 = 40000.0_f64 / 81.0_f64 * t15064 * t58581 + 160000.0_f64 / 243.0_f64 * t15064 * t58585 + t58591 + 4.0_f64 / 9.0_f64 * t43649 + 200.0_f64 / 27.0_f64 * t53390 * t5229 + 400.0_f64 / 27.0_f64 * t4297 * t58596 - 8.0_f64 * t4281 * t4289 * t11894 * t58322 - t58629 + 32.0_f64 / 9.0_f64 * t53432 + 8.0_f64 / 9.0_f64 * t53443 - 16.0_f64 / 9.0_f64 * t53445;
    (t58629, t58633)
}
