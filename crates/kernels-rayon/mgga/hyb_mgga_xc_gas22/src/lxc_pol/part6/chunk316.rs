//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 316/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk316(t1025: f64, t1028: f64, t1050: f64, t1075: f64, t1082: f64, t1090: f64, t1097: f64, t221: f64, t479: f64, t492: f64) -> f64 {
    let t1100 = 0.53237641966666666666e-3_f64 * t221 * t1025 * t479 + 1.0_f64 * t1075 * t1082 - t1028 - t1050 + 0.18311447306006545054e-3_f64 * t221 * t1025 * t492 + 0.5848223622634646207e0_f64 * t1090 * t1097;
    t1100
}
