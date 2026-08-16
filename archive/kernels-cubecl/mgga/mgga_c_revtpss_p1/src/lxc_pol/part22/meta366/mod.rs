//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1903;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1904;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1905;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta366<F: Float>(t1121: F, t3603: F, t606: F, t221: F, t462: F, t68: F, t461: F, t1209: F, t3766: F, t5330: F, t1214: F, t11772: F, t3623: F, t3717: F, t1263: F, t675: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12839, t12840, t12851, t12853, t12854, t12855) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1903::<F>(t1121, t3603, t606, t221, t462, t68, t461, t1209, t3766, t5330);
        let (t12856, t12865) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1904::<F>(t1214, t3603, t11772, t3623);
        let t12866 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1905::<F>(t12865, t3717);
        let t12879 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1906::<F>(t1263, t675);
    (t12839, t12840, t12851, t12853, t12854, t12855, t12856, t12865, t12866, t12879)
}
