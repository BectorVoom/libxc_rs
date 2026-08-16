//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1006/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1006(t3839: f64, t39875: f64, t3814: f64, t39684: f64, t3851: f64, t40897: f64, t25525: f64, t40901: f64, t5169: f64, t649: f64, t36107: f64, t36119: f64, t41000: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41142 = t3839 * t39875;
    let t41144 = t3814 * t39684;
    let t41146 = t3851 * t40897;
    let t41148 = t25525 * t40901;
    let t41150 = t649 * t5169;
    let t41151 = t36107 * t41150;
    let t41153 = t36119 * t41000;
    (t41142, t41144, t41146, t41148, t41150, t41151, t41153)
}
