//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 992/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk992(t77941: f64, t71951: f64, t352: f64, t5148: f64, t77901: f64, t71960: f64, t76236: f64, t14509: f64, t8672: f64, t14512: f64, t8533: f64, t2447: f64, t664: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77942 = 0.20455996240684006296e-1_f64 * t77941;
    let t77943 = 0.79828278012425390426e-1_f64 * t71951;
    let t77945 = t5148 * t77901 * t352;
    let t77946 = 0.2993560425465952141e-1_f64 * t77945;
    let t77949 = 0.79828278012425390426e-1_f64 * t71960;
    let t77950 = 0.18183107769496894487e-1_f64 * t76236;
    let t77954 = t14509 * t8672;
    let t77955 = 0.36366215538993788971e-1_f64 * t77954;
    let t77956 = t14512 * t8533;
    let t77957 = 0.18183107769496894486e-1_f64 * t77956;
    let t77960 = t2447 * t664;
    (t77942, t77943, t77946, t77949, t77950, t77955, t77957, t77960)
}
