//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1975/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1975(t87140: f64, t87153: f64, t87155: f64, t2627: f64, t7823: f64, t24273: f64, t2633: f64, t26654: f64, t26661: f64, t2679: f64, t4166: f64, t7837: f64, t808: f64, t812: f64, t81595: f64, t81600: f64, t81602: f64, t84851: f64, t87117: f64, t87124: f64, t87133: f64, t87150: f64, t87159: f64, t9612: f64) -> f64 {
    let t92513 = 0.3289868133696452873e-1_f64 * t87140;
    let t92515 = 0.16449340668482264365e-1_f64 * t87153;
    let t92516 = 0.52089578783527170489e-1_f64 * t87155;
    let t92521 = t2627 * t7823;
    let t92528 = 0.6579736267392905746e-1_f64 * t87117 - 0.3289868133696452873e-1_f64 * t81595 - 0.6579736267392905746e-1_f64 * t87124 - t84851 + 0.10417915756705434098e0_f64 * t81600 + 0.25587863262083522346e0_f64 * t81602 + 0.6579736267392905746e-1_f64 * t87133 + t92513 + 0.3289868133696452873e-1_f64 * t87150 - t92515 + t92516 + 0.6579736267392905746e-1_f64 * t87159 - t9612 * t7837 - t812 * t26661 * t2679 + 2.0_f64 * t812 * t92521 * t2633 + 2.0_f64 * t808 * t26654 - t4166 * t24273;
    t92528
}
