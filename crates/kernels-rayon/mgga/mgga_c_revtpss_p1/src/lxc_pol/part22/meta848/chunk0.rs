//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2987/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2987(t5600: f64, t9292: f64, t1893: f64, t4075: f64, t786: f64, t9682: f64, t10115: f64, t1894: f64, t14094: f64, t2435: f64, t1358: f64, t2439: f64, t5710: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49468 = t9292 * t5600;
    let t49471 = t786 * t1893 * t4075;
    let t49472 = t49471 * t9682;
    let t49474 = t10115 * t1894;
    let t49476 = t2435 * t14094;
    let t49480 = t2439 * t785 * t5710 * t1358;
    (t49468, t49471, t49472, t49474, t49476, t49480)
}
