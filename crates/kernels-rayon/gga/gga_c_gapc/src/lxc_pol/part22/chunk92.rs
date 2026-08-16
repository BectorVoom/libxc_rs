//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 92/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk92(t40: f64, t117: f64, zeta_threshold: f64) -> f64 {
    let t225 = 2.0_f64 <= zeta_threshold;
    let t228 = 0.0_f64 <= zeta_threshold;
    let t278 = t40 * t40;
    let t279 = piecewise3(t225, t117, t278);
    let t280 = piecewise3(t228, t117, 0.0_f64);
    let t282 = t279 / 2.0_f64 + t280 / 2.0_f64;
    t282
}
