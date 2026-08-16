//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 884/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk884(t14125: f64, t21708: f64, t9110: f64, t15211: f64, t68528: f64, t21709: f64, t68448: f64, t74312: f64, t68455: f64, t9117: f64, t21719: f64, t9188: f64, t9193: f64) -> (f64, f64, f64, f64, f64) {
    let t75806 = t21708 * t14125 * t9110;
    let t75808 = t68528 * t15211;
    let t75811 = t68448 * t21709 * t74312;
    let t75814 = t68455 * t21709 * t9117;
    let t75818 = t21719 * t9188 * t9193;
    (t75806, t75808, t75811, t75814, t75818)
}
