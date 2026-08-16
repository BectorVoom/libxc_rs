//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta657 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2112;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2113;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2114;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2115;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta657<F: Float>(t1468: F, t4343: F, t5962: F, t605: F, t6075: F, t775: F, t25207: F, t1583: F, t580: F, t98631: F, t27382: F, t29694: F, t689: F, t93314: F, t29682: F, t92838: F, t93302: F, t1032: F, t6041: F, t867: F, t786: F, t7060: F, t92843: F, t29658: F, t686: F, t72: F, t7058: F, t7064: F, t27349: F, t92858: F, t93349: F, t98803: F, t98806: F, t98811: F, t98814: F, t98817: F, t99414: F, t27186: F, t99404: F, t98849: F, t18785: F, t7053: F, t92861: F, t92870: F, t92873: F, t92875: F, t98825: F, t98830: F, t98851: F, t98853: F, t98856: F, t98858: F, t98868: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t105909, t105919, t105923, t105924, t105930, t105933) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2112::<F>(t1468, t4343, t5962, t605, t6075, t775, t25207, t1583, t580, t98631, t27382, t29694, t689);
        let (t105934, t105937, t105939, t105944, t105945, t105947, t105949) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2113::<F>(t105933, t93314, t29682, t689, t92838, t93302, t1032, t6041, t867, t786, t7060, t92843);
        let t105958 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2114::<F>(t29658, t686, t72, t7058, t7064, t105934, t105937, t105939, t105947, t105949, t27349, t92858, t93349, t98803, t98806, t98811, t98814, t98817, t99414);
        let t105969 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2115::<F>(t27186, t99404, t98849, t18785, t7053, t92861, t92870, t92873, t92875, t98825, t98830, t98851, t98853, t98856, t98858, t98868);
    (t105909, t105919, t105923, t105924, t105930, t105944, t105945, t105958, t105969)
}
