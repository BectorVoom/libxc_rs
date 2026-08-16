//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2084/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2084(t3355: f64, t427: f64, t3358: f64, t11292: f64, t1143: f64, t1124: f64, t11419: f64, t11282: f64, t43689: f64, t440: f64, t43776: f64, t43819: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44175 = t3355 * t3355;
    let t44177 = t427 / t44175;
    let t44178 = t3358 * t3358;
    let t44179 = 1.0_f64 / t44178;
    let t44205 = t1143 * t11292;
    let t44214 = t1124 * t11419;
    let t44220 = t1143 * t11282;
    let t44223 = t440 * t43689;
    let t44249 = 0.16979925925925925926e1_f64 * t43776;
    let t44275 = 0.5356037037037037037e1_f64 * t43819;
    (t44177, t44179, t44205, t44214, t44220, t44223, t44249, t44275)
}
