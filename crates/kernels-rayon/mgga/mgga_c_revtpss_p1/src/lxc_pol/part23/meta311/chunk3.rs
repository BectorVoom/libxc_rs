//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1594/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1594(t12987: f64, t480: f64, t1224: f64, t3362: f64, t1226: f64, t697: f64, t1222: f64, t12268: f64, t3698: f64, t3367: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12988 = t12987 * t480;
    let t13006 = t1224 * t3362;
    let t13011 = t697 * t1226;
    let t13012 = t1222 * t13011;
    let t13020 = t3698 * t12268;
    let t13026 = 1.0_f64 / t404 / t3367;
    (t12988, t13006, t13011, t13012, t13020, t13026)
}
