//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 860/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk860(t15093: f64, t2048: f64, t25640: f64, t74973: f64, t3826: f64, t75302: f64, t1614: f64, t3046: f64, t3851: f64, t3839: f64, t75373: f64, t75298: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t75393 = t15093 * t2048;
    let t75395 = t25640 * t74973;
    let t75397 = t3826 * t75302;
    let t75399 = t3046 * t1614;
    let t75400 = t3851 * t75399;
    let t75402 = t3851 * t75302;
    let t75405 = t3826 * t75399;
    let t75407 = t3839 * t75373;
    let t75409 = t3839 * t75298;
    (t75393, t75395, t75397, t75399, t75400, t75402, t75405, t75407, t75409)
}
