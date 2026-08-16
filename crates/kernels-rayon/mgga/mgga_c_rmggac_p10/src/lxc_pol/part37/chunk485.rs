//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 485/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk485(t1326: f64, t13897: f64, t13916: f64, t3093: f64, t7348: f64, t13912: f64, t3839: f64, t3826: f64, t2048: f64, t851: f64, t3046: f64, t328: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13917 = t1326 * t13897;
    let t13918 = t13916 * t13917;
    let t13920 = t3093 * t7348;
    let t13922 = t3839 * t13912;
    let t13924 = t3826 * t13897;
    let t13926 = t851 * t2048;
    let t13928 = t3826 * t3046;
    let t13929 = t13928 * t328;
    (t13917, t13918, t13920, t13922, t13924, t13926, t13928, t13929)
}
