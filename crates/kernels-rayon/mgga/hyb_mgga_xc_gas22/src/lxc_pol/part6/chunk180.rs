//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 180/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk180(t531: f64, t537: f64, t510: f64, t513: f64, t518: f64, t521: f64, t524: f64, t532: f64, t536: f64, t459: f64, t3: f64, t5: f64, param_c_os_0: f64) -> (f64, f64, f64, f64, f64) {
    let t538 = t537 * t531;
    let t541 = param_c_os_0 + t510 * t513 + t518 * t521 + t524 * t532 / 2.0_f64 + t536 * t538 / 2.0_f64;
    let t543 = 1.0_f64 / t459;
    let t544 = t3 * t543;
    let t545 = t5 - t544;
    (t538, t541, t543, t544, t545)
}
