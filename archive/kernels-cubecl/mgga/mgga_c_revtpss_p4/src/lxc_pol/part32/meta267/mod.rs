//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta267 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1124;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1125;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1126;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1127;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta267<F: Float>(t1497: F, t84: F, t77: F, t1470: F, t603: F, t1493: F, t76: F, t1518: F, t94: F, t1513: F, t6998: F, t1544: F, t30: F, t1549: F, t7025: F, t1561: F, t7038: F, t1565: F, t7045: F, t1568: F, t1955: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7705, t7706) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1124::<F>(t1497, t84, t77);
        let t7709 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1125::<F>(t1470, t603);
        let t7719 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1126::<F>(t1493, t76);
        let t7732 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1127::<F>(t1518, t94);
        let (t7738, t7749, t7753, t7755, t7757, t7766) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1128::<F>(t1513, t6998, t1544, t30, t1549, t7025, t1561, t7038, t1565, t7045, t1568, t1955);
    (t7705, t7706, t7709, t7719, t7732, t7738, t7749, t7753, t7755, t7757, t7766)
}
