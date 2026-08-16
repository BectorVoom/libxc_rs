//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1088/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1088(t50: f64, t238: f64, t34: f64, t821: f64, t12177: f64, t1289: f64, t15095: f64, t1699: f64, t1702: f64, t2868: f64, t2910: f64, t35: f64, t4084: f64, t5468: f64, t5493: f64, t5498: f64, t595: f64, t829: f64, t830: f64, t833: f64, zeta_threshold: f64) -> (f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t19487 = t238 * t34 * t821;
    let t19508 = piecewise3(t51, 0.0_f64, -56.0_f64 / 81.0_f64 * t12177 * t1699 * t830 - 64.0_f64 / 27.0_f64 * t4084 * t19487 + 8.0_f64 / 27.0_f64 * t5493 * t833 - 16.0_f64 / 9.0_f64 * t829 * t35 * t595 + 8.0_f64 / 9.0_f64 * t1289 * t821 - 8.0_f64 / 3.0_f64 * t1289 * t2868 + 8.0_f64 / 27.0_f64 * t2910 * t1702 * t830 - 4.0_f64 / 9.0_f64 * t829 * t5468 * t238 - 2.0_f64 / 9.0_f64 * t5498 * t833 - t15095);
    (t19487, t19508)
}
