//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 116/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk116(t360: f64, t34: f64, t35: f64, rho0: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t361 = t360 - 1.0_f64;
    let t362 = 1.0_f64 / t361;
    let t363 = sigma0 * sigma0;
    let t364 = t362 * t363;
    let t365 = t34 * t34;
    let t366 = t365 * rho0;
    let t368 = 1.0_f64 / t35 / t366;
    (t361, t362, t363, t364, t368)
}
