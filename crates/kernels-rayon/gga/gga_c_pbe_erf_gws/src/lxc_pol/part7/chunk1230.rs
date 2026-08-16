//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1230/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1230(t21596: f64, t21600: f64, t21608: f64, t21614: f64, t21616: f64, t21627: f64, t21632: f64, t21635: f64, t21640: f64, t21651: f64, t21653: f64, t2418: f64, t353: f64, t814: f64, t859: f64) -> (f64, f64) {
    let t21712 = t21596 - t21600 - t21608 + t21614 - t21616 + t21627 + t21632 - t21635 + t21640 - t21651 - t21653;
    let t21724 = t859 * t353 * t2418 * t814;
    (t21712, t21724)
}
