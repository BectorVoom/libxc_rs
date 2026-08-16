//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 484/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk484(t3046: f64, t321: f64, t1326: f64, t13911: f64, t1322: f64, t3839: f64) -> (f64, f64, f64, f64) {
    let t13912 = t3046 * t321;
    let t13913 = t1326 * t13912;
    let t13914 = t13911 * t13913;
    let t13916 = t3839 * t1322;
    (t13912, t13913, t13914, t13916)
}
