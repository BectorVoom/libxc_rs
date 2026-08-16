//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 665/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk665(t7: f64, t3282: f64, t675: f64, t1318: f64, t764: f64, t26: f64, t1794: f64, t1329: f64, t222: f64, t568: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t3283 = t3282 * t675;
    let t3287 = t764 * t1318;
    let t3288 = t26 * t3287;
    let t3293 = 2.0_f64 * t1794;
    let t3294 = piecewise3(t8, 0.0_f64, t3293);
    let t3300 = t222 * t568 * t1329;
    (t3283, t3287, t3288, t3293, t3294, t3300)
}
