//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 636/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk636(t27: f64, t615: f64, t271: f64, t71: f64, t198: f64, t202: f64, t3127: f64, t14113: f64, t14123: f64) -> (f64, f64, f64, f64, f64) {
    let t17881 = t27 * t615;
    let t20925 = t271 * t71;
    let t21052 = t198 * t202;
    let t21060 = t198 * t3127;
    let t21708 = t14113 * t14123;
    (t17881, t20925, t21052, t21060, t21708)
}
