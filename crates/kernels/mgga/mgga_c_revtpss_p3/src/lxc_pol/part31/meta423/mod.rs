//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1516;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1517;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1518;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta423<F: Float>(t6132: F, t698: F, t6135: F, t18946: F, t930: F, t141: F, t6138: F, t18942: F, t18937: F, t2908: F, t11134: F, t11366: F, t11479: F, t11480: F, t18948: F, t15123: F, t15125: F, t15128: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18951: F, t18977: F, t18980: F, t18982: F, t18985: F, t18988: F, t18990: F, t18993: F, t18995: F, t964: F, t973: F, t981: F, t3022: F, t6227: F, t11528: F, t6110: F, t2869: F, t6142: F, t11560: F, t15189: F, t15483: F, t15484: F, t15485: F, t18944: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19002, t19004, t19007, t19009, t19014, t19017, t19019) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1516::<F>(t6132, t698, t6135, t18946, t930, t141, t6138, t18942, t18937, t2908, t11134, t11366, t11479, t11480, t18948);
        let t19021 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1517::<F>(t15123, t15125, t15128, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18951, t18977, t18980, t18982, t18985, t18988, t18990, t18993, t18995, t19019);
        let (t19025, t19027, t19029, t19031, t19045) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1518::<F>(t19021, t964, t973, t981, t3022, t6227, t11528, t6110, t2869, t6142, t11134, t11560, t15189, t15483, t15484, t15485, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
    (t19002, t19004, t19007, t19009, t19014, t19017, t19021, t19025, t19027, t19029, t19031, t19045)
}
