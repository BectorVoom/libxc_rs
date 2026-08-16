//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta47 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk322;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk323;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk324;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk325;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk326;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk327;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta47(t915: f64, t936: f64, t902: f64, t908: f64, t307: f64, t302: f64, t928: f64, t919: f64, t924: f64, t932: f64, t310: f64, t324: f64, t320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t938, t941) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk322(t915, t936, t902, t908);
        let (t944, t945, t946, t953) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk323(t307, t302, t902, t928, t908, t919, t924, t932);
        let t954 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk324(t310);
        let t955 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk325(t953, t954);
        let t960 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk326(t902, t908);
        let (t961, t963, t964) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk327(t324, t960, t320);
    (t938, t941, t944, t945, t946, t953, t954, t955, t960, t961, t963, t964)
}
