//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 859/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk859(t2035: f64, t898: f64, t41: f64, t5883: f64, t5885: f64, t1745: f64, t963: f64, t5609: f64, t5612: f64, t5614: f64, t5669: f64, t5678: f64, t5682: f64, t5689: f64, t5868: f64) -> f64 {
    let t7794 = t898 * t2035;
    let t7795 = t41 * t7794;
    let t7796 = 4.0_f64 * t5883;
    let t7797 = 12.0_f64 * t5885;
    let t7798 = t963 * t1745;
    let t7800 = t5609 + t5612 - t5614 - t7795 + t5868 - t7796 - t5669 - t5678 - t5682 - t5689 + t7797 + 0.5848223622634646207e0_f64 * t7798;
    t7800
}
