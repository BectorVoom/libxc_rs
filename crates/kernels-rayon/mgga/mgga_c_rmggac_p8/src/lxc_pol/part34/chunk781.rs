//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 781/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk781(t14125: f64, t21713: f64, t9095: f64, t9137: f64, t21718: f64, t3352: f64, t8516: f64, t8518: f64, t15318: f64, t68432: f64, t16503: f64, t16504: f64, t665: f64, t9151: f64) -> (f64, f64, f64, f64, f64) {
    let t74075 = t21713 * t14125 * t9095;
    let t74078 = t21713 * t14125 * t9137;
    let t74082 = t8516 * t21718 * t3352 * t8518;
    let t74084 = t68432 * t15318;
    let t74088 = t16503 * t16504 * t665 * t9151;
    (t74075, t74078, t74082, t74084, t74088)
}
