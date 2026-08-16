//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 963/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk963(t5169: f64, t649: f64, t36107: f64, t36119: f64, t41000: f64, t25636: f64, t40901: f64, t2347: f64, t25525: f64, t794: f64, t3839: f64, t40905: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41150 = t649 * t5169;
    let t41151 = t36107 * t41150;
    let t41153 = t36119 * t41000;
    let t41155 = t25636 * t40901;
    let t41158 = t25525 * t2347 * t794;
    let t41160 = t3839 * t40905;
    (t41150, t41151, t41153, t41155, t41158, t41160)
}
