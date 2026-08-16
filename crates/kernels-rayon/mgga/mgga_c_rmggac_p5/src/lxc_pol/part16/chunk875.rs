//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 875/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk875(t40918: f64, t40970: f64, t40976: f64, t41041: f64, t41057: f64, t41114: f64, t41128: f64, t41438: f64, t2227: f64, t551: f64, t1614: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44075 = 0.10909864661698136692e0_f64 * t40918;
    let t44093 = 0.10909864661698136692e0_f64 * t40970;
    let t44095 = 0.1454648621559751559e0_f64 * t40976;
    let t44110 = 0.36366215538993788974e-1_f64 * t41041;
    let t44114 = 0.10909864661698136692e0_f64 * t41057;
    let t44143 = 0.15965655602485078085e0_f64 * t41114;
    let t44145 = 0.3193131120497015617e0_f64 * t41128;
    let t44169 = 0.3193131120497015617e0_f64 * t41438;
    let t44187 = t2227 * t551;
    let t44194 = t698 * t1614;
    (t44075, t44093, t44095, t44110, t44114, t44143, t44145, t44169, t44187, t44194)
}
