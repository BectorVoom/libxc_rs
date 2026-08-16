//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 87/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk87(t109: f64, t209: f64, t116: f64, t208: f64, t193: f64, t198: f64, t202: f64, t94: f64) -> (f64, f64, f64, f64) {
    let t210 = t109 * t209;
    let t211 = t116 + t208;
    let t212 = 1.0_f64 / t211;
    let t214 = t94 + 0.3840616724010807e-2_f64 * t193 * t198 * t202 + t210 * t212;
    (t210, t211, t212, t214)
}
