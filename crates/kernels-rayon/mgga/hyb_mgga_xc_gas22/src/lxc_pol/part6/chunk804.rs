//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 804/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk804(t2705: f64, t2709: f64, t2716: f64, t2726: f64, t2734: f64, t2737: f64, t2741: f64, t2753: f64, t2756: f64, t2759: f64, t2762: f64, t2815: f64, t4482: f64, t462: f64) -> f64 {
    let t4484 = t462 * t4482 - t2705 + t2709 - t2716 + t2726 + t2734 - t2737 - t2741 + t2753 + t2756 + t2759 - t2762 + t2815;
    t4484
}
