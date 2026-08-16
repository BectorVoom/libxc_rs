//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 617/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk617(t50: f64, t2876: f64, t478: f64, t34: f64, t893: f64, t238: f64, t821: f64, t1369: f64, t1372: f64, t39: f64, t52: f64, t830: f64, t833: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t4011 = t2876 * t478;
    let t4014 = t893 * t34;
    let t4015 = t821 * t238;
    let t4025 = piecewise3(t51, 0.0_f64, -8.0_f64 / 27.0_f64 * t4011 * t830 - 16.0_f64 / 9.0_f64 * t4014 * t4015 + 4.0_f64 / 9.0_f64 * t1369 * t833 - 8.0_f64 / 3.0_f64 * t52 * t821 + 8.0_f64 * t1372 * t39);
    (t4011, t4015, t4025)
}
