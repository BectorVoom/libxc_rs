//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 536/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk536(t1085: f64, t2704: f64, t1077: f64, t156: f64, t1084: f64, t402: f64, t474: f64, t14: f64, t25: f64, t2: f64, t39: f64, t717: f64, t732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2705 = t2704 * t1085;
    let t2706 = 0.032530742648344574_f64 * t2705;
    let t2707 = t156 * t1077;
    let t2708 = t1084 * t2707;
    let t2709 = 0.032530742648344574_f64 * t2708;
    let t2710 = t474 * t402;
    let t2711 = t1084 * t2710;
    let t2712 = 0.021687161765563047_f64 * t2711;
    let t2715 = 1.0_f64 / t14 / t25 / 4.0_f64;
    let t2716 = t2715 * t2;
    let t2717 = t2716 * t39;
    let t2719 = t732 * t717;
    (t2705, t2706, t2707, t2708, t2709, t2710, t2711, t2712, t2715, t2716, t2717, t2719)
}
