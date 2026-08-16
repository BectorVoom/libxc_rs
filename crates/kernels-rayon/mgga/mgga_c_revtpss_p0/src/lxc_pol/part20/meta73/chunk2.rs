//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 473/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk473(t2258: f64, t36: f64, t70: f64, t607: f64, t627: f64, t362: f64, t41: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t2259 = t36 * t2258;
    let t2260 = t2259 * t70;
    let t2263 = t607 * t627;
    let t2269 = 1.0_f64 / t41 / t362;
    let t2270 = sigma0 * t2269;
    (t2259, t2260, t2263, t2269, t2270)
}
