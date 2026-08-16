//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1212/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1212(t7942: f64, t7971: f64, t3017: f64, t6012: f64, t33: f64, t39: f64, t6022: f64, t6025: f64, t1179: f64, t545: f64, t1894: f64, t6033: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23319 = t7942 * t7971;
    let t23321 = t6012 * t3017;
    let t23323 = t33 * param_hyb_omega_0;
    let t23328 = t6022 * t39 * t6025;
    let t23329 = t1179 * t545;
    let t23335 = t1894 * t39 * t6033;
    (t23319, t23321, t23323, t23328, t23329, t23335)
}
