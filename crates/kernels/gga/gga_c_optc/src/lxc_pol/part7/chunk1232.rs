//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1232/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1232<F: Float>(t25423: F, t8126: F, t7373: F, t896: F, t19: F, t769: F, t7493: F, t2721: F, t8040: F, t8152: F, t10888: F, t10913: F, t10926: F, t11473: F, t23825: F, t25388: F, t25401: F, t25406: F, t25414: F, t25417: F, t25419: F, t2668: F, t2671: F, t2797: F, t2812: F, t2813: F, t3907: F, t7380: F, t7405: F, t7482: F, t7995: F, t8041: F, t8114: F, t8134: F, t8149: F, t8157: F, t914: F, t930: F) -> (F, F) {
    let t25424 = t8126 * t25423;
    let t25425 = t896 * t7373;
    let t25427 = t19 * t769;
    let t25428 = t25427 * t7493;
    let t25433 = t2721 * t8152 * t8040;
    let t25439 = F::new(0.9291736872898228042e2) * t3907 * t7995 * t25388 - F::new(0.30972456242994093474e2) * t2668 * t7995 * t10913 - F::new(0.5392791351917231181e5) * t8134 * t2671 * t10926 + F::new(0.59919903910191457566e4) * t8114 * t2671 * t10888 + F::new(0.2339219295794108718e2) * t2812 * t2813 * t25401 + F::new(0.15454509315180013964e0) * t25406 - F::new(0.13909058383662012568e1) * t930 * t914 * t7405 * t23825 - F::new(0.1039653020352937208e2) * t25414 - F::new(0.6237918122117623248e2) * t25417 + F::new(0.779739765264702906e2) * t11473 * t7482 * t25419 + F::new(0.31957282085435444036e5) * t25424 * t25425 * t7380 * t25428 - F::new(0.3029360340401625103e1) * t25433 - F::new(0.12363607452144011171e1) * t2797 * t8157 + F::new(0.24234882723213000824e2) * t8149 * t8041;
    (t25425, t25439)
}
