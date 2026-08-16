//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1111/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1111(t2288: f64, t2295: f64, t2317: f64, t2249: f64, t2278: f64, t18439: f64, t18442: f64, t6141: f64, t828: f64, t2189: f64, t2196: f64, t6352: f64, t862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18711 = t2288 * t2295;
    let t18740 = t2288 * t2317;
    let t18747 = t2249 * t2278;
    let t18750 = 0.17757530864197530864e0_f64 * t18439;
    let t18765 = 0.5356037037037037037e1_f64 * t18439;
    let t18766 = 0.16979925925925925926e1_f64 * t18442;
    let t18790 = t828 * t6141;
    let t18796 = t2189 * t2196;
    let t18799 = t6352 * t862;
    (t18711, t18740, t18747, t18750, t18765, t18766, t18790, t18796, t18799)
}
