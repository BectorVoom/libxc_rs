//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1187/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1187(t222: f64, t567: f64, t7440: f64, t7444: f64, t1057: f64, t7313: f64, t2643: f64, t7242: f64, t1068: f64, t7544: f64, t1097: f64, t1110: f64, t2647: f64, t7410: f64) -> (f64, f64, f64, f64, f64) {
    let t21994 = 0.3684616320282908548e2_f64 * t222 * t567 * t7440 * t7444;
    let t21997 = t1057 * t7313;
    let t21999 = t2643 * t7242;
    let t22004 = t7544 * t1068;
    let t22009 = 0.46785788981077169656e1_f64 * t1110 * t2647 * t7410 * t1097;
    (t21994, t21997, t21999, t22004, t22009)
}
