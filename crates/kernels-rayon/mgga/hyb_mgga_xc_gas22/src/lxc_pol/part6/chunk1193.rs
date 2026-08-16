//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1193/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1193(t21833: f64, t2729: f64, t2731: f64, t1068: f64, t7539: f64, t2754: f64, t2814: f64, t2751: f64, t221: f64, t2631: f64, t2696: f64, t1025: f64, t2630: f64, t7249: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22094 = 0.48245938496077605201e2_f64 * t2729 * t21833 * t2731;
    let t22095 = t7539 * t1068;
    let t22102 = t2754 * t2814;
    let t22105 = 120.0_f64 * t2751 * t2814;
    let t22107 = t2696 * t221 * t2631;
    let t22112 = 0.1301229756036208781e0_f64 * t2630 * t1025 * t7249;
    (t22094, t22095, t22102, t22105, t22107, t22112)
}
