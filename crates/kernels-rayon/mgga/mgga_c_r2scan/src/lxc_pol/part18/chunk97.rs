//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 97/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk97(t106: f64, t288: f64, t97: f64, rho0: f64, tau0: f64) -> (f64, f64, f64) {
    let t290 = t97 * t106 * t288;
    let t291 = pow_1_3(rho0);
    let t292 = t291 * t291;
    let t294 = 1.0_f64 / t292 / rho0;
    let t295 = tau0 * t294;
    (t290, t292, t295)
}
