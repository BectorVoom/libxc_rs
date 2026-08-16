//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 811/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk811(t2332: f64, t899: f64, t900: f64, t907: f64, t2277: f64, t6656: f64, t6663: f64, t6667: f64, t6676: f64, t6682: f64, t6685: f64, t6688: f64, t6692: f64, t6696: f64, t6700: f64, t6704: f64, t6709: f64, t6713: f64, t6714: f64) -> (f64, f64) {
    let t6717 = t899 * t900 * t2332;
    let t6718 = t6717 * t907;
    let t6720 = -35.0_f64 / 384.0_f64 * t6656 - t6663 + t2277 * t6667 / 768.0_f64 + t6676 + t6682 + 3.0_f64 / 256.0_f64 * t6685 * t6688 + t6692 - t6696 - t6700 - t6704 - t6709 - t6713 - 7.0_f64 / 768.0_f64 * t6714 + 119.0_f64 / 2304.0_f64 * t6718;
    (t6717, t6720)
}
