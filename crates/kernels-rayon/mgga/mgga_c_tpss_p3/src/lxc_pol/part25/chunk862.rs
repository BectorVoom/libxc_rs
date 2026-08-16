//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 862/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk862(t46: f64, t47: f64, t58: f64, t59: f64, t7585: f64, t2458: f64, t78: f64, t2839: f64, t81: f64, t2211: f64, t719: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7737 = 1.0_f64 / t47 / t46;
    let t7750 = 1.0_f64 / t59 / t58;
    let t7761 = 1232.0_f64 / 27.0_f64 * t7585;
    let t7771 = 1.0_f64 / t78 / t2458;
    let t7780 = 1.0_f64 / t81 / t2839;
    let t7813 = t2211 * t719;
    (t7737, t7750, t7761, t7771, t7780, t7813)
}
