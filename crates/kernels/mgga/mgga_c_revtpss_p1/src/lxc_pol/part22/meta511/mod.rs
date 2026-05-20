//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2262;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2263;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2264;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2265;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2266;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2267;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2268;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta511<F: Float>(t16725: F, t3360: F, t128: F, t2258: F, t5046: F, t2251: F, t1120: F, t3367: F, t4186: F, t606: F, t5051: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16726, t16727) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2262::<F>(t16725, t3360, t128);
        let t16729 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2263::<F>(t2258, t5046);
        let (t16730, t16731) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2264::<F>(t16729, t3360, t128);
        let t16733 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2265::<F>(t2251, t5046);
        let (t16734, t16735) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2266::<F>(t1120, t16733, t128);
        let t16738 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2267::<F>(t3367, t4186, t606);
        let (t16739, t16740) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2268::<F>(t1120, t16738, t128);
        let t16742 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2269::<F>(t2258, t5051);
    (t16726, t16727, t16729, t16730, t16731, t16733, t16734, t16735, t16738, t16739, t16740, t16742)
}
