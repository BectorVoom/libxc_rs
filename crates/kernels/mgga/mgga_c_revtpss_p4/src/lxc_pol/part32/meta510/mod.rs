//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1801;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1802;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1803;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta510<F: Float>(t30286: F, t30312: F, t532: F, t1450: F, t2071: F, t29591: F, t26550: F, t29682: F, t1579: F, t7997: F, t7071: F, t1580: F, t25391: F, t26437: F, t26439: F, t26508: F, t26521: F, t27199: F, t28315: F, t28317: F, t28352: F, t28361: F, t28366: F, t28369: F, t28371: F, t28374: F, t28391: F, t28394: F, t6049: F, t6072: F, t7070: F, t7403: F, t8012: F, t2061: F, t6071: F, t26462: F, t26468: F, t26471: F, t27228: F, t27230: F, t27256: F, t29623: F, t29627: F, t29629: F, t29631: F, t29633: F, t26450: F, t26454: F, t26457: F, t27240: F, t27246: F, t27251: F, t27254: F, t29616: F, t29618: F, t29620: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t30313, t30314, t30315, t30317, t30337, t30341, t30342, t30355) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1801::<F>(t30286, t30312, t532, t1450, t2071, t29591, t26550, t29682, t1579, t7997, t7071, t1580, t25391, t26437, t26439, t26508, t26521, t27199, t28315, t28317, t28352, t28361, t28366, t28369, t28371, t28374, t28391, t28394, t6049, t6072, t7070, t7403, t8012);
        let (t30356, t30357, t30378) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1802::<F>(t2061, t6071, t7071, t26462, t26468, t26471, t27228, t27230, t27256, t29623, t29627, t29629, t29631, t29633);
        let t30379 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1803::<F>(t26450, t26454, t26457, t27240, t27246, t27251, t27254, t29616, t29618, t29620, t30378);
    (t30313, t30314, t30315, t30317, t30337, t30341, t30342, t30355, t30356, t30357, t30379)
}
