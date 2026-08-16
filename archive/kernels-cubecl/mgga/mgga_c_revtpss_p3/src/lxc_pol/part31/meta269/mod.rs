//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta269 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1203;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1204;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1205;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1206;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta269<F: Float>(t3: F, t7318: F, t1459: F, t2042: F, t116: F, t1936: F, param_d: F, t670: F, t572: F, t117: F, t7002: F, t1461: F, t2040: F, t573: F, t38: F, t4173: F, t1497: F, t84: F, t77: F, t1470: F, t603: F, t1469: F, t6968: F, t6971: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7319, t7324, t7329, t7330) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1203::<F>(t3, t7318, t1459, t2042, t116, t1936, param_d);
        let (t7331, t7334, t7337, t7702) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1204::<F>(t670, t7330, t572, t117, t7002, t1461, t2040, t573, t7324, t7329, t38, t4173);
        let (t7705, t7706) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1205::<F>(t1497, t84, t77);
        let t7709 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1206::<F>(t1470, t603);
        let (t7714, t7715) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1207::<F>(t1469, t6968, t6971, t72);
    (t7319, t7324, t7330, t7331, t7334, t7337, t7702, t7705, t7706, t7709, t7714, t7715)
}
