//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 541/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk541(t132: f64, t1341: f64, t638: f64, t7310: f64, t2085: f64, t2181: f64, t33: f64, t78: f64, t271: f64, t4765: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7311 = t132 * t1341;
    let t7313 = t638 * t7310 * t7311;
    let t7318 = t2181 * t2085;
    let t7319 = 0.18183107769496894486e-1_f64 * t7318;
    let t7320 = t78 * t33;
    let t7321 = 1.0_f64 / t7320;
    let t7322 = t7321 * t271;
    let t7323 = t4765 * t7322;
    (t7311, t7313, t7319, t7321, t7322, t7323)
}
