//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 981/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk981(t77749: f64, t15624: f64, t321: f64, t3352: f64, t515: f64, t7230: f64, t1971: f64, t2144: f64, t333: f64, t352: f64, t875: f64, t118: f64, t1986: f64, t615: f64, t699: f64) -> (f64, f64, f64, f64, f64) {
    let t77750 = 0.53205749866622299248e-5_f64 * t77749;
    let t77754 = t7230 * t3352 * t515 * t15624 * t321;
    let t77755 = 0.15961724959986689774e-4_f64 * t77754;
    let t77759 = t7230 * t1971 * t2144 * t15624 * t333;
    let t77760 = 0.15961724959986689774e-4_f64 * t77759;
    let t77764 = t7230 * t1971 * t875 * t15624 * t352;
    let t77765 = 0.1064114997332445985e-4_f64 * t77764;
    let t77768 = t1986 * t118 * t699 * t615;
    (t77750, t77755, t77760, t77765, t77768)
}
