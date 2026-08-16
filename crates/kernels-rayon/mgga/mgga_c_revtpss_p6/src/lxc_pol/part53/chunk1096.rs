//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1096/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1096(t120000: f64, t817: f64, t8485: f64, t2718: f64, t8479: f64, t31830: f64, t119825: f64, t25412: f64, t240: f64, t27: f64, t822: f64, t119967: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120002 = t120000 * t8485 * t817;
    let t120004 = t8479 * t2718;
    let t120005 = t31830 * t120004;
    let t120006 = t119825 * t25412;
    let t120007 = t120005 * t120006;
    let t120010 = t822 * t27 * t240;
    let t120011 = t119967 * t120010;
    (t120002, t120004, t120005, t120006, t120007, t120010, t120011)
}
