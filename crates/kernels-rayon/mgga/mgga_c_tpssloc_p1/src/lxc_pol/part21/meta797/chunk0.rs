//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2765/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2765(t16752: f64, t252: f64, t4233: f64, t232: f64, t13170: f64, t2632: f64, t829: f64, t13397: f64, t13453: f64, t16758: f64, t16805: f64, t16815: f64, t16816: f64, t17030: f64, t17037: f64, t2684: f64, t40951: f64, t4162: f64, t4182: f64, t4280: f64, t4281: f64, t4282: f64, t4283: f64, t4291: f64, t58166: f64, t812: f64, t860: f64, t863: f64, t9632: f64) -> (f64, f64, f64, f64, f64) {
    let t58262 = t252 * t16752;
    let t58280 = t4233 * t4233;
    let t58281 = t58280 * t232;
    let t58289 = t2632 * t13170;
    let t58300 = t829 * t4233;
    let t58304 = -24.0_f64 * t13397 * t16816 * t4233 * t4282 - 6.0_f64 * t13397 * t16815 * t40951 + 4.0_f64 * t16758 * t4281 * t9632 - t17030 * t2684 * t4291 + 2.0_f64 * t17030 * t4281 * t9632 + 8.0_f64 * t4162 * t4280 * t4283 + 12.0_f64 * t4182 * t4281 * t58166 + 4.0_f64 * t4182 * t4281 * t58262 + 4.0_f64 * t4281 * t4282 * t58289 - 4.0_f64 * t4282 * t4291 * t58300 - 2.0_f64 * t58281 * t812 * t860 + 8.0_f64 * t13453 * t17037 + 2.0_f64 * t16805 * t863;
    (t58262, t58280, t58281, t58289, t58304)
}
