//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1002/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1002(t858: f64, t8989: f64, t3065: f64, t8988: f64, t343: f64, t8827: f64, t6672: f64, t2169: f64, t887: f64, t856: f64, t3108: f64, t8958: f64, t8960: f64, t8965: f64, t8969: f64, t8971: f64, t8973: f64, t8977: f64, t8980: f64, t8985: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8990 = t858 * t8989;
    let t8991 = t3065 * t8990;
    let t8993 = t8988 * t8991 / 24.0_f64;
    let t8994 = t8827 * t343;
    let t8995 = t858 * t8994;
    let t8996 = t3065 * t8995;
    let t8998 = t6672 * t8996 / 48.0_f64;
    let t8999 = t2169 * t887;
    let t9000 = t856 * t8999;
    let t9002 = t3108 * t9000 / 24.0_f64;
    let t9003 = -t8958 + t8960 - t8965 - t8969 + t8971 + t8973 - t8977 + t8980 + t8985 + t8993 - t8998 - t9002;
    (t8991, t8993, t8996, t8998, t9002, t9003)
}
