//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1263/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1263(t54285: f64, t54289: f64, t54301: f64, t54319: f64, t54322: f64, t54329: f64, t54344: f64, t54354: f64, t54377: f64, t54397: f64, t54401: f64, t14937: f64, t9270: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t55570 = 7.0_f64 / 72.0_f64 * t54285;
    let t55572 = 7.0_f64 / 72.0_f64 * t54289;
    let t55580 = 7.0_f64 / 288.0_f64 * t54301;
    let t55591 = 7.0_f64 / 36.0_f64 * t54319;
    let t55593 = 7.0_f64 / 36.0_f64 * t54322;
    let t55596 = 7.0_f64 / 12.0_f64 * t54329;
    let t55603 = 35.0_f64 / 144.0_f64 * t54344;
    let t55608 = 7.0_f64 / 144.0_f64 * t54354;
    let t55620 = 7.0_f64 / 36.0_f64 * t54377;
    let t55633 = 7.0_f64 / 72.0_f64 * t54397;
    let t55634 = 7.0_f64 / 72.0_f64 * t54401;
    let t55660 = 7.0_f64 / 72.0_f64 * t9270 * t14937;
    (t55570, t55572, t55580, t55591, t55593, t55596, t55603, t55608, t55620, t55633, t55634, t55660)
}
