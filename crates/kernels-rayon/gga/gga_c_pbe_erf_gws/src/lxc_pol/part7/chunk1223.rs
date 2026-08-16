//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1223/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1223(t6116: f64, t840: f64, t329: f64, t340: f64, t6593: f64, t847: f64, t20255: f64, t20258: f64, t20261: f64, t20278: f64, t20280: f64, t20284: f64, t20301: f64, t20321: f64, t20328: f64, t20335: f64, t20357: f64) -> (f64, f64, f64) {
    let t21674 = t840 * t6116;
    let t21681 = t329 * t6593 * t340;
    let t21682 = t21681 * t847;
    let t21687 = t20255 - t20258 - t20261 - t20278 - t20280 + t20284 - t20301 - t20321 + t20328 + t20335 - t20357;
    (t21674, t21682, t21687)
}
