//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 579/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk579(t1052: f64, t1101: f64, t1068: f64, t1057: f64, t1067: f64, t1100: f64) -> (f64, f64, f64, f64) {
    let t2737 = 8.0_f64 * t1052 * t1101;
    let t2738 = t1052 * t1068;
    let t2741 = 8.0_f64 * t1057 * t1101;
    let t2742 = t1067 * t1100;
    (t2737, t2738, t2741, t2742)
}
