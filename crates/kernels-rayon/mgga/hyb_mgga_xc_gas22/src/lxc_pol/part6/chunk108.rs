//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 108/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk108(t228: f64, t231: f64, t234: f64, t245: f64) -> (f64, f64, f64) {
    let t267 = 0.705945e1_f64 * t231 + 0.1549425e1_f64 * t228 + 0.420775e0_f64 * t234 + 0.1562925e0_f64 * t245;
    let t270 = 1.0_f64 + 0.32163958997385070134e2_f64 / t267;
    let t271 = f64::ln(t270);
    (t267, t270, t271)
}
