//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta657 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2112;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2113;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2114;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2115;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta657(t1468: f64, t4343: f64, t5962: f64, t605: f64, t6075: f64, t775: f64, t25207: f64, t1583: f64, t580: f64, t98631: f64, t27382: f64, t29694: f64, t689: f64, t93314: f64, t29682: f64, t92838: f64, t93302: f64, t1032: f64, t6041: f64, t867: f64, t786: f64, t7060: f64, t92843: f64, t29658: f64, t686: f64, t72: f64, t7058: f64, t7064: f64, t27349: f64, t92858: f64, t93349: f64, t98803: f64, t98806: f64, t98811: f64, t98814: f64, t98817: f64, t99414: f64, t27186: f64, t99404: f64, t98849: f64, t18785: f64, t7053: f64, t92861: f64, t92870: f64, t92873: f64, t92875: f64, t98825: f64, t98830: f64, t98851: f64, t98853: f64, t98856: f64, t98858: f64, t98868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t105909, t105919, t105923, t105924, t105930, t105933) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2112(t1468, t4343, t5962, t605, t6075, t775, t25207, t1583, t580, t98631, t27382, t29694, t689);
        let (t105934, t105937, t105939, t105944, t105945, t105947, t105949) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2113(t105933, t93314, t29682, t689, t92838, t93302, t1032, t6041, t867, t786, t7060, t92843);
        let t105958 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2114(t29658, t686, t72, t7058, t7064, t105934, t105937, t105939, t105947, t105949, t27349, t92858, t93349, t98803, t98806, t98811, t98814, t98817, t99414);
        let t105969 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2115(t27186, t99404, t98849, t18785, t7053, t92861, t92870, t92873, t92875, t98825, t98830, t98851, t98853, t98856, t98858, t98868);
    (t105909, t105919, t105923, t105924, t105930, t105944, t105945, t105958, t105969)
}
