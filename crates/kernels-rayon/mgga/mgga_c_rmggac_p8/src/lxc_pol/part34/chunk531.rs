//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 531/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk531(t14267: f64, t3056: f64, t641: f64, t2046: f64, t2049: f64, t2604: f64, t3072: f64, t13989: f64, t7788: f64, t13993: f64, t7782: f64, t14004: f64, t7835: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14269 = t3056 * t14267 * t641;
    let t14272 = t2046 * t2049 * t641;
    let t14274 = t2604 * t3072;
    let t14275 = 0.2993560425465952141e-1_f64 * t14274;
    let t14276 = t7788 * t13989;
    let t14278 = t7782 * t13993;
    let t14280 = t7835 * t14004;
    (t14269, t14272, t14275, t14276, t14278, t14280)
}
