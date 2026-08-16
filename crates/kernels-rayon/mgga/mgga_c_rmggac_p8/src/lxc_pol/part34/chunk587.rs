//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 587/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk587(t1326: f64, t15105: f64, t13911: f64, t15098: f64, t13916: f64, t3839: f64, t3826: f64, t13928: f64, t556: f64, t13931: f64, t2842: f64, t13937: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15106 = t1326 * t15105;
    let t15107 = t13911 * t15106;
    let t15109 = t1326 * t15098;
    let t15110 = t13916 * t15109;
    let t15112 = t3839 * t15105;
    let t15114 = t3826 * t15098;
    let t15116 = t13928 * t556;
    let t15118 = t13931 * t2842;
    let t15120 = t13937 * t15106;
    (t15106, t15107, t15109, t15110, t15112, t15114, t15116, t15118, t15120)
}
