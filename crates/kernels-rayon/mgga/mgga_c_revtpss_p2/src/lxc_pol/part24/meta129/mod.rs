//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk681;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk682;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk683;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk684;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta129(t354: f64, t357: f64, t3298: f64, t378: f64, t342: f64, t3154: f64, t3302: f64, t3316: f64, t1678: f64, t359: f64, t198: f64, t336: f64, t1716: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4975, t4980) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk681(t354, t357, t3298, t378);
        let (t4981, t4982, t4995) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk682(t342, t4980, t3154, t3302, t3316, t378);
        let (t4996, t5004) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk683(t342, t4995, t1678, t359);
        let t5023 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk684(t198, t336);
        let t5044 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk685(t1716, t689);
    (t4975, t4980, t4981, t4982, t4995, t4996, t5004, t5023, t5044)
}
