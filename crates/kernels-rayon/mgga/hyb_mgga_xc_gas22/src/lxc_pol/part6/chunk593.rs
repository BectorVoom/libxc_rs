//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 593/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk593(t2814: f64, t462: f64, t2341: f64, t2628: f64, t2737: f64, t2738: f64, t2741: f64, t2743: f64, t2745: f64, t2747: f64, t2753: f64, t2756: f64, t2759: f64, t2762: f64) -> (f64, f64) {
    let t2815 = t462 * t2814;
    let t2816 = t462 * t2747 - t2341 - t2628 + t2737 + 8.0_f64 * t2738 - t2741 + 2.0_f64 * t2743 - 8.0_f64 * t2745 + t2753 + t2756 - t2759 - t2762 + t2815;
    (t2815, t2816)
}
