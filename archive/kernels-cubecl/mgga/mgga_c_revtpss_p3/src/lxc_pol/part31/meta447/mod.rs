//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1600;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1601;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1602;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta447<F: Float>(t20266: F, t3360: F, t128: F, t3367: F, t5825: F, t606: F, t1120: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t12296: F, t12297: F, t16706: F, t16915: F, t16916: F, t16917: F, t1132: F, t1145: F, t141: F, t20302: F, t3417: F, t20298: F, t20310: F, t20306: F, t12327: F, t6442: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t20315 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1600::<F>(t20266, t3360, t128);
        let (t20318, t20320) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1601::<F>(t3367, t5825, t606, t1120, t128);
        let (t20322, t20337) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1602::<F>(t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320, t12296, t12297, t16706, t16915, t16916, t16917);
        let (t20338, t20341, t20344, t20347, t20350, t20353, t20356) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1603::<F>(t1132, t20337, t1145, t20318, t141, t20302, t3417, t20298, t20310, t20306, t12327, t6442);
    (t20315, t20318, t20320, t20322, t20337, t20338, t20341, t20344, t20347, t20350, t20353, t20356)
}
