//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 806/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk806(t12323: f64, t247: f64, t251: f64, t10607: f64, t10611: f64, t10633: f64, t12592: f64, t12593: f64, t12595: f64, t12598: f64, t12601: f64, t12602: f64, t12603: f64, t12604: f64, t12605: f64, t12607: f64, t12608: f64, t12611: f64, t12615: f64, t12619: f64, t256: f64) -> (f64, f64, f64) {
    let t13008 = t12323 * t247;
    let t13009 = t13008 * t251;
    let t13013 = -t12592 + t12593 + t13009 * t256 / 3.0_f64 + t12595 - t12598 - t12601 + t12602 + t12603 - t12604 - t12605 + t10607 + 0.18233333333333333333e0_f64 * t10611 + t12607 - t12608 + t10633 - t12611 + t12615 - t12619;
    (t13008, t13009, t13013)
}
