//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 890/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk890(t1322: f64, t235: f64, t29837: f64, t15144: f64, t352: f64, t1326: f64, t27: f64, t9145: f64, t16129: f64, t70489: f64, t1469: f64, t34976: f64, t39851: f64, t665: f64) -> (f64, f64, f64, f64, f64) {
    let t75961 = t235 * t29837 * t1322;
    let t75962 = t15144 * t352;
    let t75963 = t1326 * t75962;
    let t75964 = t75961 * t75963;
    let t75966 = t27 * t9145;
    let t75968 = t70489 * t16129 * t75966;
    let t75972 = t39851 * t34976 * t665 * t1469;
    (t75962, t75963, t75964, t75968, t75972)
}
