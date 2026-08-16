//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta277 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1233;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1234;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1235;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1236;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta277<F: Float>(t1927: F, t8143: F, t2122: F, t7719: F, t5: F, t1923: F, t2123: F, t7566: F, t7702: F, t7706: F, t7709: F, t117: F, t30: F, t265: F, t393: F, t1518: F, t2163: F, t7855: F, t1469: F, t2129: F, t45: F, t7794: F, t1479: F, t343: F, t136: F, t1785: F, t2138: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1802: F, t2137: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t8144 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1233::<F>(t1927, t8143);
        let t8147 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1234::<F>(t2122, t7719);
        let (t8151, t8152) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1235::<F>(t5, t1923, t2123, t7566, t7702, t7706, t7709, t8144, t8147, t117);
        let (t8158, t8161, t8166, t8171, t8172, t8177) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1236::<F>(t30, t265, t393, t1518, t2163, t7855, t1469, t2129, t45, t7794, t1479, t343, t136, t1785, t2138, dens_threshold, rho0, zeta_threshold);
        let t8184 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1237::<F>(t1802, t2137);
    (t8144, t8147, t8151, t8152, t8158, t8161, t8166, t8171, t8172, t8177, t8184)
}
