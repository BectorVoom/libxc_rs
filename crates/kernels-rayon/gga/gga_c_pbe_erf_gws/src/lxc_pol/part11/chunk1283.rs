//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1283/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1283(t49625: f64, t49634: f64, t49641: f64, t49643: f64, t49650: f64, t49652: f64, t49658: f64, t49660: f64, t49661: f64, t49663: f64, t49664: f64, t49667: f64, t49671: f64, t49672: f64, t49673: f64, t49681: f64, t49683: f64, t49687: f64, t49696: f64, t49717: f64, t49722: f64, t49729: f64) -> (f64, f64) {
    let t50572 = t49625 - t49634 - t49641 - t49643 + t49650 + t49652 + t49658 - t49660 + t49661 - t49663 - t49664;
    let t50574 = -t49667 - t49671 - t49672 + t49673 - t49681 - t49683 + t49687 + t49696 - t49717 + t49722 - t49729;
    (t50572, t50574)
}
