//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta277 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1241;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1242;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1243;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1244;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta277<F: Float>(t1916: F, t2042: F, t1518: F, t7330: F, t572: F, t117: F, t7741: F, t1918: F, t2040: F, t573: F, t7944: F, t3140: F, t3268: F, t1078: F, t1035: F, t2033: F, t4147: F, t587: F, t65: F, t197: F, t532: F, t1450: F, t143: F, t2580: F, t130: F, t2566: F, t700: F, t2584: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7950, t7953, t7956, t8515) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1241::<F>(t1916, t2042, t1518, t7330, t572, t117, t7741, t1918, t2040, t573, t7944, t3140, t3268);
        let t8521 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1242::<F>(t1078, t3140, t1035);
        let t8717 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1243::<F>(t2033, t4147);
        let (t8779, t8995, t8996, t9274, t9275, t9276) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1244::<F>(t587, t65, t197, t532, t1450, t2033, t143, t2580, t130, t2566, t700, t2584);
        let t9278 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1245::<F>(t9274, t9276);
    (t7950, t7953, t7956, t8515, t8521, t8717, t8779, t8995, t8996, t9275, t9278)
}
