//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 49/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk49(t101: f64, param_c_x_2: f64, sigma0: f64) -> (f64, f64, f64, f64) {
    let t102 = sigma0 * t101;
    let t104 = 1.0_f64 + 0.3840616724010807e-2_f64 * t102;
    let t105 = 1.0_f64 / t104;
    let t109 = param_c_x_2;
    (t102, t104, t105, t109)
}
