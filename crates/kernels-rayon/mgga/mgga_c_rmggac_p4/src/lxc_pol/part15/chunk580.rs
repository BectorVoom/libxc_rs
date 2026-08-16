//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 580/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk580(t2127: f64, t290: f64, t236: f64, t830: f64, t507: f64, t2004: f64, t2186: f64, t2007: f64, t1223: f64, t28: f64, t212: f64, t672: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7894 = t290 * t2127;
    let t7900 = t236 * t830;
    let t7901 = t507 * t7900;
    let t7908 = t2186 * t2004;
    let t7909 = 0.19863479950205658386e-4_f64 * t7908;
    let t7910 = t2186 * t2007;
    let t7919 = t1223 * t28;
    let t7920 = t212 * t7919;
    let t7921 = t672 * t7920;
    (t7894, t7900, t7901, t7909, t7910, t7920, t7921)
}
