//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2120;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2121;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2122;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2123;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta454<F: Float>(t15220: F, t923: F, t916: F, t11134: F, t11136: F, t11138: F, t11140: F, t11339: F, t11366: F, t11368: F, t11479: F, t11480: F, t11326: F, t15108: F, t15111: F, t15114: F, t15116: F, t15119: F, t15121: F, t15123: F, t15125: F, t15128: F, t15132: F, t15175: F, t15178: F, t15181: F, t15184: F, t15187: F, t15189: F, t15192: F, t15195: F, t15198: F, t15200: F, t973: F, t2962: F, t4673: F, t11452: F, t1621: F, t2944: F, t4708: F, t972: F, t1634: F, t3006: F, t2988: F, t4711: F, t3014: F, t4707: F, t11450: F, t11461: F, t11466: F, t11554: F, t15100: F, t15103: F, t15104: F, t2945: F, t2968: F, t2987: F, t3012: F, t4690: F, t4712: F, t965: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15221, t15230, t15232) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2120::<F>(t15220, t923, t916, t11134, t11136, t11138, t11140, t11339, t11366, t11368, t11479, t11480);
        let t15234 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2121::<F>(t11326, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15125, t15128, t15132, t15175, t15178, t15181, t15184, t15187, t15189, t15192, t15195, t15198, t15200, t15232);
        let (t15235, t15238, t15242, t15249, t15252, t15255) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2122::<F>(t15234, t973, t2962, t4673, t11452, t1621, t2944, t4708, t972, t1634, t3006, t2988, t4711);
        let (t15258, t15259, t15262) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2123::<F>(t3014, t4707, t972, t11450, t11461, t11466, t11554, t15100, t15103, t15104, t15235, t15238, t15242, t15249, t15252, t15255, t2945, t2968, t2987, t3012, t4690, t4712, t965);
    (t15221, t15230, t15234, t15235, t15238, t15242, t15249, t15252, t15255, t15258, t15259, t15262)
}
