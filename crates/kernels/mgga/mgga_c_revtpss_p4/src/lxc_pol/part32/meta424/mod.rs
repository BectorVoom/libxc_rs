//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1503;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1504;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1505;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1506;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta424<F: Float>(t4186: F, t5051: F, t1120: F, t128: F, t20266: F, t3360: F, t3367: F, t5825: F, t606: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t12296: F, t12297: F, t16706: F, t16915: F, t16916: F, t16917: F) -> (F, F, F, F, F, F, F) {
        let (t20310, t20312) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1503::<F>(t4186, t5051, t1120, t128);
        let t20315 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1504::<F>(t20266, t3360, t128);
        let (t20318, t20320) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1505::<F>(t3367, t5825, t606, t1120, t128);
        let (t20322, t20337) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1506::<F>(t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320, t12296, t12297, t16706, t16915, t16916, t16917);
    (t20310, t20312, t20315, t20318, t20320, t20322, t20337)
}
