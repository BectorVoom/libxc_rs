//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta658 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2116;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2117;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2118;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2119;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2120;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2121;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2122;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2123;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2124;
use chunk9::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2125;
use chunk10::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2126;
use chunk11::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta658<F: Float>(t29643: F, t686: F, t72: F, t93281: F, t93317: F, t18451: F, t25270: F, t18462: F, t18647: F, t18527: F, t98988: F, t18471: F, t18446: F, t18629: F, t18428: F, t27261: F, t18651: F, t18639: F, t98937: F, t98950: F, t18643: F, t92955: F, t18456: F, t6037: F, t92951: F, t18521: F, t25222: F, t6030: F, t103264: F, t92963: F, t92966: F, t92969: F, t92976: F, t98968: F, t98973: F, t18423: F, t25234: F, t5993: F, t103269: F, t103270: F, t103285: F, t92989: F, t92991: F, t98984: F, t98992: F, t99001: F, t99002: F, t99007: F, t18414: F, t2661: F, t93082: F, t18418: F, t25227: F, t18398: F, t7045: F, t18402: F, t18409: F, t25266: F, t5980: F, t18482: F, t18478: F, t18531: F, t25245: F, t18432: F, t93025: F, t18440: F, t103287: F, t99012: F, t18437: F, t18348: F, t1945: F, t807: F, t6019: F, t6024: F, t93054: F, t103297: F, t99020: F, t99022: F, t99024: F, t99027: F, t99030: F, t99034: F, t99042: F, t18495: F, t18500: F, t18618: F, t7038: F, t18466: F, t103302: F, t103305: F, t92996: F, t92998: F, t93000: F, t93001: F, t93008: F, t93013: F, t93016: F, t18622: F, t5989: F, t92978: F, t18634: F, t18334: F, t25277: F, t5985: F, t93021: F, t93035: F, t99066: F, t99070: F, t99074: F, t99078: F, t99086: F, t18394: F, t7025: F, t27221: F, t62403: F, t18352: F, t61639: F, t99062: F, t61725: F, t103329: F, t103347: F, t93049: F, t93067: F, t93073: F, t93088: F, t99100: F, t99103: F, t1580: F, t213: F, t225: F, t25322: F, t257: F, t6049: F, t92895: F, t92905: F, t98875: F, t98879: F, t98881: F, t98894: F, t98897: F, t98907: F, t98911: F, t99429: F, t29654: F, t25387: F, t25375: F, t29610: F, t27183: F, t27199: F, t92935: F, t93112: F, t93116: F, t93138: F, t93142: F, t98918: F, t98920: F, t99127: F, t99147: F, t99163: F, t99166: F) -> (F, F, F, F) {
        let (t105974, t105976, t105985, t105987, t105989, t105991, t105993) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2116::<F>(t29643, t686, t72, t93281, t93317, t18451, t25270, t18462, t18647, t18527, t98988, t18471);
        let t106005 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2117::<F>(t18446, t25270, t18629, t18428, t27261, t18651, t18639, t105985, t105987, t105989, t105991, t105993, t98937, t98950);
        let t106020 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2118::<F>(t18643, t92955, t18456, t27261, t6037, t92951, t18521, t25222, t6030, t103264, t92963, t92966, t92969, t92976, t98968, t98973);
        let t106028 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2119::<F>(t18423, t25234, t25222, t5993, t103269, t103270, t103285, t92989, t92991, t98984, t98992, t99001, t99002, t99007);
        let (t106030, t106033, t106035, t106037, t106040, t106042) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2120::<F>(t18414, t2661, t93082, t18418, t25227, t18398, t7045, t18402, t25234, t18409, t25266, t5980);
        let t106055 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2121::<F>(t18482, t25270, t18478, t27261, t18531, t25245, t18432, t93025, t18440, t25227, t2661, t103287, t106030, t106033, t106035, t106037, t106040, t106042, t99012);
        let t106067 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2122::<F>(t18437, t7045, t18348, t1945, t807, t25266, t6019, t6024, t93054, t103297, t99020, t99022, t99024, t99027, t99030, t99034, t99042);
        let t106078 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2123::<F>(t18495, t7045, t18500, t18618, t7038, t18466, t25270, t103302, t103305, t92996, t92998, t93000, t93001, t93008, t93013, t93016);
        let t106092 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2124::<F>(t18622, t25245, t5989, t92978, t18634, t27261, t18334, t25270, t25277, t5985, t93021, t93035, t99066, t99070, t99074, t99078, t99086);
        let t106108 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2125::<F>(t18394, t7025, t27221, t62403, t18352, t1945, t807, t61639, t99062, t61725, t103329, t103347, t93049, t93067, t93073, t93088, t99100, t99103);
        let (t106111, t106116) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2126::<F>(t106005, t106020, t106028, t106055, t106067, t106078, t106092, t106108, t105974, t105976, t1580, t213, t225, t25322, t257, t6049, t92895, t92905, t98875, t98879, t98881, t98894, t98897, t98907, t98911, t99429);
        let (t106128, t106134) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2127::<F>(t29654, t686, t72, t25387, t25375, t29610, t27183, t27199, t92935, t93112, t93116, t93138, t93142, t98918, t98920, t99127, t99147, t99163, t99166);
    (t106111, t106116, t106128, t106134)
}
