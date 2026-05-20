//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta190 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk900;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk901;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk902;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk903;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta190<F: Float>(t1469: F, t3367: F, t606: F, t1120: F, t128: F, t1121: F, t4186: F, t3357: F, t3358: F, t5044: F, t5049: F, t422: F, t1130: F, t1719: F, t1151: F, t1733: F, t3379: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5051, t5052) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk900::<F>(t1469, t3367, t606);
        let (t5053, t5054) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk901::<F>(t1120, t5052, t128);
        let t5056 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk902::<F>(t1121, t4186);
        let (t5057, t5058) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk903::<F>(t1120, t5056, t128);
        let (t5060, t5062, t5063, t5065, t5067) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk904::<F>(t3357, t3358, t5044, t5049, t5054, t5058, t422, t1130, t1719, t1151, t1733, t3379);
    (t5051, t5052, t5053, t5054, t5056, t5057, t5058, t5060, t5062, t5063, t5065, t5067)
}
