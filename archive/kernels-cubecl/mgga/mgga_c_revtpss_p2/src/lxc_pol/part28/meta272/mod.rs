//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1218;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1219;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1220;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta272<F: Float>(t1470: F, t603: F, t1469: F, t6968: F, t6971: F, t72: F, t1927: F, t1493: F, t76: F, t1926: F, t5: F, t1923: F, t1928: F, t6958: F, t7702: F, t7706: F, t117: F, t1937: F, t4248: F, t1518: F, t94: F, t1843: F, t1936: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7709, t7714, t7715, t7716) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1218::<F>(t1470, t603, t1469, t6968, t6971, t72, t1927);
        let (t7719, t7720) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1219::<F>(t1493, t76, t1926);
        let (t7724, t7725, t7731, t7732) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1220::<F>(t5, t1923, t1928, t6958, t7702, t7706, t7709, t7716, t7720, t117, t1937, t4248, t1518, t94);
        let (t7734, t7735) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1221::<F>(t1937, t7732, t1843, t1936);
    (t7709, t7714, t7715, t7716, t7719, t7720, t7724, t7725, t7731, t7732, t7734, t7735)
}
