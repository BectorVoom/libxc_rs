//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 914/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk914(t1725: f64, t29195: f64, t2418: f64, t8729: f64, t23528: f64, t2417: f64, t10925: f64, t10983: f64, t1706: f64, t17520: f64, t17567: f64, t23496: f64, t29196: f64, t29228: f64, t29231: f64, t29244: f64, t45: f64, t4858: f64, t4909: f64, t634: f64, t7091: f64, t8698: f64, t8730: f64, t8733: f64) -> f64 {
    let t29250 = t29195 * t1725;
    let t29253 = t2418 * t8729;
    let t29256 = t23528 * t2417;
    let t29259 = 3.0_f64 * t23496 * t2418 + 3.0_f64 * t7091 * t8730 + 0.48245472966453314466e2_f64 * t17567 * t8733 - 0.96490945932906628932e2_f64 * t10983 * t29196 + 1.0_f64 * t1706 * t29228 + 0.51725014705706168417e3_f64 * t10925 * t29231 + 0.19751789702565206229e-1_f64 * t45 * t29244 * t634 - 6.0_f64 * t17520 * t8698 + 6.0_f64 * t4909 * t29250 - 6.0_f64 * t4858 * t29253 + 0.48245472966453314466e2_f64 * t4909 * t29256;
    t29259
}
