//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1086/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1086(t300: f64, t6404: f64, t2255: f64, t2277: f64, t356: f64, t18439: f64, t18442: f64, t6141: f64, t828: f64, t6312: f64, t858: f64, t6121: f64, t877: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18661 = t300 * t6404;
    let t18706 = t356 / t2277 / t2255;
    let t18750 = 0.17757530864197530864e0_f64 * t18439;
    let t18765 = 0.5356037037037037037e1_f64 * t18439;
    let t18766 = 0.16979925925925925926e1_f64 * t18442;
    let t18790 = t828 * t6141;
    let t18843 = 0.18467901234567901234e0_f64 * t18439;
    let t18854 = t858 * t6312;
    let t18863 = t877 * t6121;
    (t18661, t18706, t18750, t18765, t18766, t18790, t18843, t18854, t18863)
}
