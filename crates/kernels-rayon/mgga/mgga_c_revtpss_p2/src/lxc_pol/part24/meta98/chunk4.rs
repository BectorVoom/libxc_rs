//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 569/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk569(t3204: f64, t366: f64, t1014: f64, t2857: f64, t271: f64, t905: f64) -> (f64, f64, f64) {
    let t3205 = t3204 * t366;
    let t3236 = t1014 * t2857;
    let t3252 = 1.0_f64 / t271 / t905;
    (t3205, t3236, t3252)
}
