//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1036/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1036(t352: f64, t5148: f64, t77901: f64, t71960: f64, t76236: f64, t14509: f64, t8672: f64, t14512: f64, t8533: f64, t333: f64, t4669: f64, t76228: f64, t76232: f64, t77894: f64, t77933: f64, t77935: f64, t77938: f64, t77940: f64, t77942: f64, t77943: f64) -> f64 {
    let t77945 = t5148 * t77901 * t352;
    let t77946 = 0.2993560425465952141e-1_f64 * t77945;
    let t77949 = 0.79828278012425390426e-1_f64 * t71960;
    let t77950 = 0.18183107769496894487e-1_f64 * t76236;
    let t77954 = t14509 * t8672;
    let t77955 = 0.36366215538993788971e-1_f64 * t77954;
    let t77956 = t14512 * t8533;
    let t77957 = 0.18183107769496894486e-1_f64 * t77956;
    let t77958 = t77933 - t77935 + t77938 - t77940 - t77942 + t77943 + t77946 - 0.82834157616596963776e-1_f64 * t76228 - 0.16566831523319392755e-1_f64 * t76232 - t77949 - t77950 - 0.17961362552795712846e0_f64 * t4669 * t77894 * t333 + t77955 + t77957;
    t77958
}
