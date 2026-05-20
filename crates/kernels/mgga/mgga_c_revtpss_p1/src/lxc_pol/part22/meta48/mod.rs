//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta48 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk349;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk350;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk351;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk352;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk353;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk354;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta48<F: Float>(t915: F, t936: F, t902: F, t908: F, t307: F, t302: F, t928: F, t919: F, t924: F, t932: F, t310: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t938, t939, t941) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk349::<F>(t915, t936, t902, t908);
        let (t944, t945) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk350::<F>(t307);
        let t946 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk351::<F>(t302, t945);
        let (t948, t951, t953) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk352::<F>(t902, t928, t908, t919, t924, t932);
        let t954 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk353::<F>(t310);
        let t955 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk354::<F>(t953, t954);
    (t938, t939, t941, t944, t945, t946, t948, t951, t953, t954, t955)
}
