//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1134/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1134(t1139: f64, t12636: f64, t1136: f64, t1149: f64, t12555: f64, t12557: f64, t12569: f64, t12573: f64, t12577: f64, t1587: f64, t3113: f64, t3120: f64, t3145: f64, t4296: f64, t4300: f64, t4323: f64, t473: f64, t9730: f64) -> f64 {
    let t12637 = t1139 * t12636;
    let t12639 = -6.0_f64 * t1136 * t12569 + 4.0_f64 * t1136 * t12573 + 2.0_f64 * t1136 * t12577 - t1136 * t12637 - 2.0_f64 * t1149 * t12557 + t12555 * t473 - t1587 * t9730 + 4.0_f64 * t3113 * t4300 - 2.0_f64 * t3113 * t4323 + 2.0_f64 * t3120 * t4296 - t3145 * t4296;
    t12639
}
