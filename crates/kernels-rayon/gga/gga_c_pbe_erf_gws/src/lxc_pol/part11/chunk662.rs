//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 662/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk662(t2332: f64, t899: f64, t900: f64, t329: f64, t6594: f64, t378: f64, t4383: f64, t824: f64, t6472: f64, t825: f64, t332: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6717 = t899 * t900 * t2332;
    let t6729 = t329 * t6594;
    let t6731 = 455.0_f64 / 1296.0_f64 * t6729 * t378;
    let t6792 = t824 * t4383;
    let t6801 = t6472 * t825;
    let t6816 = t329 * t332 * t931;
    (t6717, t6729, t6731, t6792, t6801, t6816)
}
