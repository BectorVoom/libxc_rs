//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 422/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk422(t1775: f64, t199: f64, t1672: f64, t220: f64, t211: f64, t617: f64, t209: f64, t184: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1777 = 2.0_f64 / 15.0_f64 * t1775 * t199;
    let t1778 = t1672 * t220;
    let t1780 = 4.0_f64 / 135.0_f64 * t211 * t1778;
    let t1781 = t617 * t617;
    let t1782 = t1781 * t209;
    let t1783 = t1782 * t184;
    (t1777, t1778, t1780, t1781, t1782, t1783)
}
