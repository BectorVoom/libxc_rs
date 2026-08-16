//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2731/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2731(t15045: f64, t2435: f64, t15048: f64, t2471: f64, t15008: f64, t10996: f64, t14990: f64, t41070: f64, t14939: f64, t212: f64, t689: f64, t780: f64) -> (f64, f64, f64, f64, f64) {
    let t50218 = t2435 * t15045;
    let t50219 = 0.21951497276451705329e-1_f64 * t50218;
    let t50220 = t15048 * t2471;
    let t50221 = 0.39029762157531132076e-1_f64 * t50220;
    let t50222 = t2435 * t15008;
    let t50223 = 0.21951497276451705329e-1_f64 * t50222;
    let t50227 = t41070 * t14990 * t10996;
    let t50232 = t689 * t212 * t14939 * t780;
    (t50219, t50221, t50223, t50227, t50232)
}
