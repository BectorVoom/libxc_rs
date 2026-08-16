//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1232/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1232(t25423: f64, t8126: f64, t7373: f64, t896: f64, t19: f64, t769: f64, t7493: f64, t2721: f64, t8040: f64, t8152: f64, t10888: f64, t10913: f64, t10926: f64, t11473: f64, t23825: f64, t25388: f64, t25401: f64, t25406: f64, t25414: f64, t25417: f64, t25419: f64, t2668: f64, t2671: f64, t2797: f64, t2812: f64, t2813: f64, t3907: f64, t7380: f64, t7405: f64, t7482: f64, t7995: f64, t8041: f64, t8114: f64, t8134: f64, t8149: f64, t8157: f64, t914: f64, t930: f64) -> (f64, f64) {
    let t25424 = t8126 * t25423;
    let t25425 = t896 * t7373;
    let t25427 = t19 * t769;
    let t25428 = t25427 * t7493;
    let t25433 = t2721 * t8152 * t8040;
    let t25439 = 0.9291736872898228042e2_f64 * t3907 * t7995 * t25388 - 0.30972456242994093474e2_f64 * t2668 * t7995 * t10913 - 0.5392791351917231181e5_f64 * t8134 * t2671 * t10926 + 0.59919903910191457566e4_f64 * t8114 * t2671 * t10888 + 0.2339219295794108718e2_f64 * t2812 * t2813 * t25401 + 0.15454509315180013964e0_f64 * t25406 - 0.13909058383662012568e1_f64 * t930 * t914 * t7405 * t23825 - 0.1039653020352937208e2_f64 * t25414 - 0.6237918122117623248e2_f64 * t25417 + 0.779739765264702906e2_f64 * t11473 * t7482 * t25419 + 0.31957282085435444036e5_f64 * t25424 * t25425 * t7380 * t25428 - 0.3029360340401625103e1_f64 * t25433 - 0.12363607452144011171e1_f64 * t2797 * t8157 + 0.24234882723213000824e2_f64 * t8149 * t8041;
    (t25425, t25439)
}
