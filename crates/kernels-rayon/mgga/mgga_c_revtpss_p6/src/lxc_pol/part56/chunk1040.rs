//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1040/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1040(t120004: f64, t31830: f64, t119825: f64, t25412: f64, t240: f64, t27: f64, t822: f64, t119967: f64, t119837: f64, t14686: f64, t837: f64, t119833: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120005 = t31830 * t120004;
    let t120006 = t119825 * t25412;
    let t120007 = t120005 * t120006;
    let t120010 = t822 * t27 * t240;
    let t120011 = t119967 * t120010;
    let t120013 = t14686 * t119837 * t837;
    let t120014 = t120011 * t120013;
    let t120016 = t119833 * t120010;
    (t120005, t120006, t120007, t120011, t120013, t120014, t120016)
}
