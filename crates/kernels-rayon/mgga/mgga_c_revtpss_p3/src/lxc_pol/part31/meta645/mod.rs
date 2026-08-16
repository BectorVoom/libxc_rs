//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2105;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2106;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2107;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2108;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2109;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2110;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2111;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta645(t105928: f64, t27382: f64, t29694: f64, t689: f64, t93314: f64, t29682: f64, t92838: f64, t93302: f64, t1032: f64, t6041: f64, t867: f64, t786: f64, t7060: f64, t92843: f64, t29658: f64, t686: f64, t72: f64, t7058: f64, t7064: f64, t27349: f64, t92858: f64, t93349: f64, t98803: f64, t98806: f64, t98811: f64, t98814: f64, t98817: f64, t99414: f64, t27186: f64, t99404: f64, t98849: f64, t18785: f64, t7053: f64, t92861: f64, t92870: f64, t92873: f64, t92875: f64, t98825: f64, t98830: f64, t98851: f64, t98853: f64, t98856: f64, t98858: f64, t98868: f64, t29643: f64, t93281: f64, t93317: f64, t18451: f64, t25270: f64, t18462: f64, t18647: f64, t18527: f64, t98988: f64, t18471: f64, t18446: f64, t18629: f64, t18428: f64, t27261: f64, t18651: f64, t18639: f64, t98937: f64, t98950: f64, t18643: f64, t92955: f64, t18456: f64, t6037: f64, t92951: f64, t18521: f64, t25222: f64, t6030: f64, t103264: f64, t92963: f64, t92966: f64, t92969: f64, t92976: f64, t98968: f64, t98973: f64, t18423: f64, t25234: f64, t5993: f64, t103269: f64, t103270: f64, t103285: f64, t92989: f64, t92991: f64, t98984: f64, t98992: f64, t99001: f64, t99002: f64, t99007: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t105930, t105934, t105936, t105937, t105939, t105944, t105945, t105946) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2105(t105928, t27382, t29694, t689, t93314, t29682, t92838, t93302, t1032, t6041, t867, t786);
        let t105958 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2106(t105946, t7060, t105936, t92843, t29658, t686, t72, t7058, t7064, t105934, t105937, t105939, t27349, t92858, t93349, t98803, t98806, t98811, t98814, t98817, t99414);
        let t105969 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2107(t27186, t99404, t98849, t18785, t7053, t92861, t92870, t92873, t92875, t98825, t98830, t98851, t98853, t98856, t98858, t98868);
        let (t105974, t105976, t105985, t105987, t105989, t105991, t105993) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2108(t29643, t686, t72, t93281, t93317, t18451, t25270, t18462, t18647, t18527, t98988, t18471);
        let t106005 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2109(t18446, t25270, t18629, t18428, t27261, t18651, t18639, t105985, t105987, t105989, t105991, t105993, t98937, t98950);
        let t106020 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2110(t18643, t92955, t18456, t27261, t6037, t92951, t18521, t25222, t6030, t103264, t92963, t92966, t92969, t92976, t98968, t98973);
        let t106028 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2111(t18423, t25234, t25222, t5993, t103269, t103270, t103285, t92989, t92991, t98984, t98992, t99001, t99002, t99007);
    (t105930, t105944, t105945, t105958, t105969, t105974, t105976, t106005, t106020, t106028)
}
