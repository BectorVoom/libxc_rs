//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 864/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk864(t28455: f64, t600: f64, t1674: f64, t28338: f64, t28343: f64, t28346: f64, t28352: f64, t28354: f64, t28356: f64, t28360: f64, t28441: f64, t28444: f64, t45: f64, t6851: f64, t8592: f64) -> (f64, f64) {
    let t28456 = t28455 * t600;
    let t28459 = 0.35089340384731224426e1_f64 * t1674 * t28338 - 0.35089340384731224426e1_f64 * t1674 * t28343 - 0.51947267698127589897e2_f64 * t1674 * t28346 + 0.35089340384731224426e1_f64 * t6851 * t8592 + t28352 + t28354 + t28356 - t28360 + t28441 + t28444 + 0.19751789702565206229e-1_f64 * t45 * t28456;
    (t28456, t28459)
}
