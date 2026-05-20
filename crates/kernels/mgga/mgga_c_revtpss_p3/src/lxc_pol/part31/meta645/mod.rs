//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2105;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2106;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2107;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2108;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2109;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2110;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2111;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta645<F: Float>(t105928: F, t27382: F, t29694: F, t689: F, t93314: F, t29682: F, t92838: F, t93302: F, t1032: F, t6041: F, t867: F, t786: F, t7060: F, t92843: F, t29658: F, t686: F, t72: F, t7058: F, t7064: F, t27349: F, t92858: F, t93349: F, t98803: F, t98806: F, t98811: F, t98814: F, t98817: F, t99414: F, t27186: F, t99404: F, t98849: F, t18785: F, t7053: F, t92861: F, t92870: F, t92873: F, t92875: F, t98825: F, t98830: F, t98851: F, t98853: F, t98856: F, t98858: F, t98868: F, t29643: F, t93281: F, t93317: F, t18451: F, t25270: F, t18462: F, t18647: F, t18527: F, t98988: F, t18471: F, t18446: F, t18629: F, t18428: F, t27261: F, t18651: F, t18639: F, t98937: F, t98950: F, t18643: F, t92955: F, t18456: F, t6037: F, t92951: F, t18521: F, t25222: F, t6030: F, t103264: F, t92963: F, t92966: F, t92969: F, t92976: F, t98968: F, t98973: F, t18423: F, t25234: F, t5993: F, t103269: F, t103270: F, t103285: F, t92989: F, t92991: F, t98984: F, t98992: F, t99001: F, t99002: F, t99007: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t105930, t105934, t105936, t105937, t105939, t105944, t105945, t105946) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2105::<F>(t105928, t27382, t29694, t689, t93314, t29682, t92838, t93302, t1032, t6041, t867, t786);
        let t105958 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2106::<F>(t105946, t7060, t105936, t92843, t29658, t686, t72, t7058, t7064, t105934, t105937, t105939, t27349, t92858, t93349, t98803, t98806, t98811, t98814, t98817, t99414);
        let t105969 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2107::<F>(t27186, t99404, t98849, t18785, t7053, t92861, t92870, t92873, t92875, t98825, t98830, t98851, t98853, t98856, t98858, t98868);
        let (t105974, t105976, t105985, t105987, t105989, t105991, t105993) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2108::<F>(t29643, t686, t72, t93281, t93317, t18451, t25270, t18462, t18647, t18527, t98988, t18471);
        let t106005 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2109::<F>(t18446, t25270, t18629, t18428, t27261, t18651, t18639, t105985, t105987, t105989, t105991, t105993, t98937, t98950);
        let t106020 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2110::<F>(t18643, t92955, t18456, t27261, t6037, t92951, t18521, t25222, t6030, t103264, t92963, t92966, t92969, t92976, t98968, t98973);
        let t106028 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2111::<F>(t18423, t25234, t25222, t5993, t103269, t103270, t103285, t92989, t92991, t98984, t98992, t99001, t99002, t99007);
    (t105930, t105944, t105945, t105958, t105969, t105974, t105976, t106005, t106020, t106028)
}
