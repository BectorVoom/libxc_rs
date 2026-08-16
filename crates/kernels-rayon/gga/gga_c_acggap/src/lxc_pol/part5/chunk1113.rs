//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1113/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1113(t50: f64, t11627: f64, t1369: f64, t14837: f64, t1699: f64, t1702: f64, t19487: f64, t238: f64, t2868: f64, t2876: f64, t35: f64, t4011: f64, t5460: f64, t5465: f64, t5468: f64, t595: f64, t821: f64, t830: f64, t833: f64, t893: f64, zeta_threshold: f64) -> f64 {
    let t51 = t50 <= zeta_threshold;
    let t19966 = piecewise3(t51, 0.0_f64, 40.0_f64 / 81.0_f64 * t11627 * t1699 * t830 + 64.0_f64 / 27.0_f64 * t4011 * t19487 - 8.0_f64 / 27.0_f64 * t5460 * t833 + 32.0_f64 / 9.0_f64 * t893 * t35 * t595 - 16.0_f64 / 9.0_f64 * t1369 * t821 + 16.0_f64 / 3.0_f64 * t1369 * t2868 - 8.0_f64 / 27.0_f64 * t2876 * t1702 * t830 + 8.0_f64 / 9.0_f64 * t893 * t5468 * t238 + 4.0_f64 / 9.0_f64 * t5465 * t833 - t14837);
    t19966
}
