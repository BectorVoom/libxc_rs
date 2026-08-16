//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 950/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk950(t448: f64, t8396: f64, t315: f64, t7966: f64, t2137: f64, t7943: f64, t33428: f64, t2134: f64, t119: f64, t8993: f64, t1432: f64, t30147: f64, t30148: f64, t7842: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33795 = t8396 * t448;
    let t33796 = t315 * t33795;
    let t33798 = 0.17347256376410398924e1_f64 * t33796 * t7966;
    let t33799 = t2137 * t33795;
    let t33801 = 0.17347256376410398924e1_f64 * t33799 * t7943;
    let t33802 = t315 * t33428;
    let t33804 = 0.17347256376410398924e1_f64 * t33802 * t2134;
    let t33818 = t119 * t8993;
    let t33831 = t30147 * t7842 * t30148 * t1432;
    (t33795, t33796, t33798, t33799, t33801, t33804, t33818, t33831)
}
