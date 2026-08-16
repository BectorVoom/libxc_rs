//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1242/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1242(t45283: f64, t37507: f64, t37363: f64, t37377: f64, t49658: f64, t49660: f64, t49661: f64, t49663: f64, t49664: f64, t49667: f64, t49671: f64, t45351: f64) -> (f64, f64, f64, f64) {
    let t49672 = 7.0_f64 / 72.0_f64 * t45283;
    let t49673 = 35.0_f64 / 72.0_f64 * t37507;
    let t49674 = t49658 - t49660 + t49661 - t49663 - t49664 - 119.0_f64 / 576.0_f64 * t37363 - 119.0_f64 / 144.0_f64 * t37377 - t49667 - t49671 - t49672 + t49673;
    let t49681 = 7.0_f64 / 12.0_f64 * t45351;
    (t49672, t49673, t49674, t49681)
}
