//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1801;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1802;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1803;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta510(t30286: f64, t30312: f64, t532: f64, t1450: f64, t2071: f64, t29591: f64, t26550: f64, t29682: f64, t1579: f64, t7997: f64, t7071: f64, t1580: f64, t25391: f64, t26437: f64, t26439: f64, t26508: f64, t26521: f64, t27199: f64, t28315: f64, t28317: f64, t28352: f64, t28361: f64, t28366: f64, t28369: f64, t28371: f64, t28374: f64, t28391: f64, t28394: f64, t6049: f64, t6072: f64, t7070: f64, t7403: f64, t8012: f64, t2061: f64, t6071: f64, t26462: f64, t26468: f64, t26471: f64, t27228: f64, t27230: f64, t27256: f64, t29623: f64, t29627: f64, t29629: f64, t29631: f64, t29633: f64, t26450: f64, t26454: f64, t26457: f64, t27240: f64, t27246: f64, t27251: f64, t27254: f64, t29616: f64, t29618: f64, t29620: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30313, t30314, t30315, t30317, t30337, t30341, t30342, t30355) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1801(t30286, t30312, t532, t1450, t2071, t29591, t26550, t29682, t1579, t7997, t7071, t1580, t25391, t26437, t26439, t26508, t26521, t27199, t28315, t28317, t28352, t28361, t28366, t28369, t28371, t28374, t28391, t28394, t6049, t6072, t7070, t7403, t8012);
        let (t30356, t30357, t30378) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1802(t2061, t6071, t7071, t26462, t26468, t26471, t27228, t27230, t27256, t29623, t29627, t29629, t29631, t29633);
        let t30379 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1803(t26450, t26454, t26457, t27240, t27246, t27251, t27254, t29616, t29618, t29620, t30378);
    (t30313, t30314, t30315, t30317, t30337, t30341, t30342, t30355, t30356, t30357, t30379)
}
