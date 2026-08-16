//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1221/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1221(t1243: f64, t3150: f64, t684: f64, t2014: f64, t8493: f64, t3161: f64, t6469: f64, t8465: f64, t2024: f64, t6479: f64, t8469: f64, t8481: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23814 = t684 * t3150 * t1243;
    let t23817 = t684 * t2014 * t8493;
    let t23828 = t684 * t6469 * t3161;
    let t23831 = t684 * t2014 * t8465;
    let t23834 = t2024 * t6479 * t8469;
    let t23853 = t684 * t2014 * t8481;
    (t23814, t23817, t23828, t23831, t23834, t23853)
}
