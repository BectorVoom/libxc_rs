//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1200/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1200(t1037: f64, t1080: f64, t1082: f64, t1095: f64, t21874: f64, t21982: f64, t21994: f64, t22033: f64, t22038: f64, t22042: f64, t22045: f64, t22050: f64, t22068: f64, t22072: f64, t22076: f64, t22094: f64, t221: f64, t2639: f64, t2771: f64, t2774: f64, t2783: f64, t2788: f64, t2789: f64, t2791: f64, t2809: f64, t492: f64, t7359: f64, t7410: f64) -> f64 {
    let t22276 = 0.69263436422725855036e2_f64 * t2809 * t7410 * t2639 * t1095 + 36.0_f64 * t2789 * t2774 * t2783 + t21982 + t21994 + t22033 + t22038 + t22042 + t22045 - t22050 + 0.12865583598954028054e3_f64 * t2789 * t7359 * t2791 * t1080 - 0.18989649058080861537e-2_f64 * t221 * t21874 * t492 - t22068 + t22072 + 0.41096e0_f64 * t1037 * t2771 * t2783 * t1082 - 0.6609050294782684211e1_f64 * t1037 * t2788 * t2783 * t2791 * t1080 - t22076 - t22094;
    t22276
}
