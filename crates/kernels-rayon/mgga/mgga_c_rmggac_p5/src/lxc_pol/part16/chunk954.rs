//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 954/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk954(t1971: f64, t3351: f64, t3924: f64, t6397: f64, t39208: f64, t9147: f64, t2868: f64, t8821: f64, t8365: f64, t8672: f64, t26857: f64, t8545: f64) -> (f64, f64, f64, f64, f64) {
    let t45909 = t3351 * t1971 * t3924 * t6397;
    let t45911 = t39208 * t9147;
    let t45914 = t2868 * t8821;
    let t45916 = t8365 * t8672;
    let t45918 = t26857 * t8545;
    (t45909, t45911, t45914, t45916, t45918)
}
