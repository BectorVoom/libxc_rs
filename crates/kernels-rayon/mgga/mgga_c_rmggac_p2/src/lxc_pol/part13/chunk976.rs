//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 976/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk976(t2103: f64, t41036: f64, t2118: f64, t39680: f64, t4669: f64, t27041: f64, t38564: f64, t35959: f64, t3839: f64, t5156: f64, t649: f64, t25640: f64, t40998: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41379 = t2103 * t41036;
    let t41381 = t2118 * t41036;
    let t41393 = t4669 * t39680;
    let t41395 = t27041 * t38564;
    let t41400 = t3839 * t35959;
    let t41402 = t41400 * t649 * t5156;
    let t41404 = t25640 * t40998;
    (t41379, t41381, t41393, t41395, t41402, t41404)
}
