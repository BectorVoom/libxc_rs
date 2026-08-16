//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1025/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1025(t41164: f64, t41199: f64, t41232: f64, t41259: f64, t41293: f64, t41334: f64, t41360: f64, t41383: f64, t39680: f64, t4669: f64, t27041: f64, t38564: f64) -> (f64, f64, f64) {
    let t41386 = t41164 + t41199 + t41232 + t41259 + t41293 + t41334 + t41360 + t41383;
    let t41393 = t4669 * t39680;
    let t41395 = t27041 * t38564;
    (t41386, t41393, t41395)
}
