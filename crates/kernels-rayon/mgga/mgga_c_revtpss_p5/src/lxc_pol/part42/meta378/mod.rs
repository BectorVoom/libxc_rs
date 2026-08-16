//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1244;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1245;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta378(t6132: f64, t698: f64, t6135: f64, t18946: f64, t930: f64, t141: f64, t6138: f64, t18942: f64, t18937: f64, t2908: f64, t11134: f64, t11366: f64, t11479: f64, t11480: f64, t18948: f64, t15123: f64, t15125: f64, t15128: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18951: f64, t18977: f64, t18980: f64, t18982: f64, t18985: f64, t18988: f64, t18990: f64, t18993: f64, t18995: f64, t964: f64, t973: f64, t981: f64, t3022: f64, t6227: f64, t11528: f64, t6110: f64, t2869: f64, t6142: f64, t11560: f64, t15189: f64, t15483: f64, t15484: f64, t15485: f64, t18944: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19002, t19004, t19007, t19009, t19014, t19017, t19019) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1244(t6132, t698, t6135, t18946, t930, t141, t6138, t18942, t18937, t2908, t11134, t11366, t11479, t11480, t18948);
        let t19021 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1245(t15123, t15125, t15128, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18951, t18977, t18980, t18982, t18985, t18988, t18990, t18993, t18995, t19019);
        let (t19025, t19027, t19029, t19031, t19045) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1246(t19021, t964, t973, t981, t3022, t6227, t11528, t6110, t2869, t6142, t11134, t11560, t15189, t15483, t15484, t15485, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
    (t19002, t19004, t19007, t19009, t19014, t19017, t19021, t19025, t19027, t19029, t19031, t19045)
}
