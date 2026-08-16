//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2809/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2809(t5611: f64, t852: f64, t17022: f64, t814: f64, t13176: f64, t13390: f64, t13407: f64, t16673: f64, t16754: f64, t16762: f64, t17027: f64, t17041: f64, t226: f64, t235: f64, t2617: f64, t2679: f64, t2728: f64, t2738: f64, t4166: f64, t4281: f64, t4282: f64, t4286: f64, t4288: f64, t4291: f64, t58340: f64, t58345: f64, t59328: f64, t812: f64, t829: f64) -> (f64, f64) {
    let t59331 = t852 * t5611;
    let t59347 = t814 * t17022;
    let t59351 = -t17027 * t2679 * t812 + t226 * t235 * t59328 + 4.0_f64 * t2728 * t58340 * t812 + 24.0_f64 * t4281 * t4282 * t58345 - 2.0_f64 * t4291 * t59331 * t829 - 2.0_f64 * t59347 * t812 * t829 - 4.0_f64 * t13176 * t4286 - 4.0_f64 * t13176 * t4288 - 4.0_f64 * t13390 * t16762 - 4.0_f64 * t13407 * t4166 - t16673 * t2738 - 2.0_f64 * t16754 * t2617 - 4.0_f64 * t17041 * t2617;
    (t59331, t59351)
}
