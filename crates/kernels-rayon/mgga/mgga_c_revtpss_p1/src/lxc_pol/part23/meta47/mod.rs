//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta47 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk340;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk341;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk342;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk343;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk344;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta47(t934: f64, t935: f64, t915: f64, t902: f64, t908: f64, t307: f64, t302: f64, t928: f64, t919: f64, t924: f64, t932: f64, t310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t936, t938, t939, t941) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk340(t934, t935, t915, t902, t908);
        let (t944, t945) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk341(t307);
        let t946 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk342(t302, t945);
        let (t948, t951, t953) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk343(t902, t928, t908, t919, t924, t932);
        let t954 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk344(t310);
    (t936, t938, t939, t941, t944, t945, t946, t948, t951, t953, t954)
}
