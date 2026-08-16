//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 649/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk649(t2327: f64, t94: f64, t1310: f64, t670: f64, t112: f64, t2289: f64, t625: f64, t666: f64, t111: f64, t654: f64) -> (f64, f64, f64, f64, f64) {
    let t2328 = t94 * t2327;
    let t2331 = t1310 * t670;
    let t2335 = 11.0_f64 / 9.0_f64 * t2289 * t112;
    let t2336 = t625 * t666;
    let t2339 = 1.0_f64 / t654 / t111;
    (t2328, t2331, t2335, t2336, t2339)
}
