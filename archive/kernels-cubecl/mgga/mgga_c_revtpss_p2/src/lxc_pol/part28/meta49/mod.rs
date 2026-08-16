//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta49 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk322;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk323;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk324;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta49<F: Float>(t934: F, t935: F, t915: F, t902: F, t908: F, t307: F, t302: F, t928: F, t919: F, t924: F, t932: F, t310: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t936, t938, t939, t941, t944, t945) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk322::<F>(t934, t935, t915, t902, t908, t307);
        let t946 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk323::<F>(t302, t945);
        let (t948, t951, t953) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk324::<F>(t902, t928, t908, t919, t924, t932);
        let t954 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk325::<F>(t310);
    (t936, t938, t939, t941, t944, t945, t946, t948, t951, t953, t954)
}
