//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 814/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk814(t15405: f64, t7244: f64, t3351: f64, t498: f64, t7231: f64, t875: f64, t8936: f64, t13823: f64, t1661: f64, t7755: f64, t21719: f64, t35312: f64, t9212: f64) -> (f64, f64, f64, f64) {
    let t74669 = t7244 * t15405;
    let t74670 = 0.19863479950205658386e-4_f64 * t74669;
    let t74674 = t3351 * t7231 * t875 * t8936 * t498;
    let t74677 = t13823 * t7755 * t1661;
    let t74684 = t21719 * t35312 * t9212;
    (t74670, t74674, t74677, t74684)
}
