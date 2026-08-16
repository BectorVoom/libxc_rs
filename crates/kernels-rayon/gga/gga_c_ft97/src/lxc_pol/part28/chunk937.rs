//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 937/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk937(t136303: f64, t5612: f64, t22513: f64, t5589: f64, t14: f64, t1669: f64, t22755: f64, t92354: f64, t5522: f64, t420: f64, t5590: f64, t92461: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t136304 = t136303 * t5612;
    let t136305 = t22513 * t136304;
    let t136307 = sigma0 * t5589;
    let t136308 = t136307 * t14;
    let t136313 = t1669 * t22755 * t92354;
    let t136331 = t1669 * t5522 * t92354;
    let t136332 = t5590 * t420;
    let t136336 = t92461 * t420;
    (t136304, t136305, t136307, t136308, t136313, t136331, t136332, t136336)
}
