//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2067;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta605<F: Float>(t3572: F, t8945: F, t12657: F, t2142: F, t45551: F, t473: F, t1243: F, t2149: F, t37885: F, t3555: F, t7627: F, t1209: F, t26884: F, t26921: F, t7648: F, t3552: F, t26983: F, t7658: F, t12627: F, t7635: F, t27033: F, t3801: F, t12587: F, t7669: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t97363, t97370, t97377, t97397, t97402, t97419) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2067::<F>(t3572, t8945, t12657, t2142, t45551, t473, t1243, t2149, t37885, t3555, t7627, t1209, t26884);
        let (t97422, t97425, t97453, t97475, t97487, t97491) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2068::<F>(t26921, t7648, t2142, t3552, t26983, t7658, t12627, t7635, t27033, t3801, t12587, t7669);
    (t97363, t97370, t97377, t97397, t97402, t97419, t97422, t97425, t97453, t97475, t97487, t97491)
}
