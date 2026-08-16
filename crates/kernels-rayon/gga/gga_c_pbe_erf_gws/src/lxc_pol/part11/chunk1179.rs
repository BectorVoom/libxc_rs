//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1179/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1179(t16845: f64, t47675: f64, t47676: f64, t47677: f64, t47678: f64, t47679: f64, t47683: f64, t47684: f64, t47685: f64, t47687: f64, t47691: f64, t16876: f64, t47695: f64, t47699: f64, t47701: f64, t47706: f64, t47707: f64, t47711: f64, t47715: f64, t47719: f64, t47723: f64, t47724: f64) -> (f64, f64) {
    let t48638 = t47675 + t47676 + t47677 - t47678 + t47679 - t47683 - t16845 - t47684 + t47685 + t47687 - t47691;
    let t48640 = -t47695 - t47699 + t47701 + t47706 - t47707 + t47711 + t47715 - t47719 + t47723 - t16876 + t47724;
    (t48638, t48640)
}
