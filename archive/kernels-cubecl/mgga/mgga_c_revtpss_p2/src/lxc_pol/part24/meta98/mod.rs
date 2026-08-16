//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta98 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk565;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk566;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk567;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk568;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk569;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk570;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta98<F: Float>(t127: F, t246: F, t283: F, t905: F, t66: F, t371: F, t373: F, t676: F, t367: F, t225: F, t3057: F, t366: F, t1014: F, t2857: F, t271: F, t2852: F, t1077: F, t384: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3172 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk565::<F>(t127, t246);
        let t3181 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk566::<F>(t283, t905);
        let t3182 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk567::<F>(t3181, t66);
        let (t3201, t3203, t3204) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk568::<F>(t371, t373, t676, t367, t225, t3057);
        let (t3205, t3236, t3252) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk569::<F>(t3204, t366, t1014, t2857, t271, t905);
        let (t3253, t3268, t3269) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk570::<F>(t2852, t3252, t1077, t384, t225);
    (t3172, t3181, t3182, t3201, t3203, t3204, t3205, t3236, t3252, t3253, t3268, t3269)
}
