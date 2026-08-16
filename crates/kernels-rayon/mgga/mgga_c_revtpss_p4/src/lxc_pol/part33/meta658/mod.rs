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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta658(t29643: f64, t686: f64, t72: f64, t93281: f64, t93317: f64, t18451: f64, t25270: f64, t18462: f64, t18647: f64, t18527: f64, t98988: f64, t18471: f64, t18446: f64, t18629: f64, t18428: f64, t27261: f64, t18651: f64, t18639: f64, t98937: f64, t98950: f64, t18643: f64, t92955: f64, t18456: f64, t6037: f64, t92951: f64, t18521: f64, t25222: f64, t6030: f64, t103264: f64, t92963: f64, t92966: f64, t92969: f64, t92976: f64, t98968: f64, t98973: f64, t18423: f64, t25234: f64, t5993: f64, t103269: f64, t103270: f64, t103285: f64, t92989: f64, t92991: f64, t98984: f64, t98992: f64, t99001: f64, t99002: f64, t99007: f64, t18414: f64, t2661: f64, t93082: f64, t18418: f64, t25227: f64, t18398: f64, t7045: f64, t18402: f64, t18409: f64, t25266: f64, t5980: f64, t18482: f64, t18478: f64, t18531: f64, t25245: f64, t18432: f64, t93025: f64, t18440: f64, t103287: f64, t99012: f64, t18437: f64, t18348: f64, t1945: f64, t807: f64, t6019: f64, t6024: f64, t93054: f64, t103297: f64, t99020: f64, t99022: f64, t99024: f64, t99027: f64, t99030: f64, t99034: f64, t99042: f64, t18495: f64, t18500: f64, t18618: f64, t7038: f64, t18466: f64, t103302: f64, t103305: f64, t92996: f64, t92998: f64, t93000: f64, t93001: f64, t93008: f64, t93013: f64, t93016: f64, t18622: f64, t5989: f64, t92978: f64, t18634: f64, t18334: f64, t25277: f64, t5985: f64, t93021: f64, t93035: f64, t99066: f64, t99070: f64, t99074: f64, t99078: f64, t99086: f64, t18394: f64, t7025: f64, t27221: f64, t62403: f64, t18352: f64, t61639: f64, t99062: f64, t61725: f64, t103329: f64, t103347: f64, t93049: f64, t93067: f64, t93073: f64, t93088: f64, t99100: f64, t99103: f64, t1580: f64, t213: f64, t225: f64, t25322: f64, t257: f64, t6049: f64, t92895: f64, t92905: f64, t98875: f64, t98879: f64, t98881: f64, t98894: f64, t98897: f64, t98907: f64, t98911: f64, t99429: f64, t29654: f64, t25387: f64, t25375: f64, t29610: f64, t27183: f64, t27199: f64, t92935: f64, t93112: f64, t93116: f64, t93138: f64, t93142: f64, t98918: f64, t98920: f64, t99127: f64, t99147: f64, t99163: f64, t99166: f64) -> (f64, f64, f64, f64) {
        let (t105974, t105976, t105985, t105987, t105989, t105991, t105993) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2116(t29643, t686, t72, t93281, t93317, t18451, t25270, t18462, t18647, t18527, t98988, t18471);
        let t106005 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2117(t18446, t25270, t18629, t18428, t27261, t18651, t18639, t105985, t105987, t105989, t105991, t105993, t98937, t98950);
        let t106020 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2118(t18643, t92955, t18456, t27261, t6037, t92951, t18521, t25222, t6030, t103264, t92963, t92966, t92969, t92976, t98968, t98973);
        let t106028 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2119(t18423, t25234, t25222, t5993, t103269, t103270, t103285, t92989, t92991, t98984, t98992, t99001, t99002, t99007);
        let (t106030, t106033, t106035, t106037, t106040, t106042) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2120(t18414, t2661, t93082, t18418, t25227, t18398, t7045, t18402, t25234, t18409, t25266, t5980);
        let t106055 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2121(t18482, t25270, t18478, t27261, t18531, t25245, t18432, t93025, t18440, t25227, t2661, t103287, t106030, t106033, t106035, t106037, t106040, t106042, t99012);
        let t106067 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2122(t18437, t7045, t18348, t1945, t807, t25266, t6019, t6024, t93054, t103297, t99020, t99022, t99024, t99027, t99030, t99034, t99042);
        let t106078 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2123(t18495, t7045, t18500, t18618, t7038, t18466, t25270, t103302, t103305, t92996, t92998, t93000, t93001, t93008, t93013, t93016);
        let t106092 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2124(t18622, t25245, t5989, t92978, t18634, t27261, t18334, t25270, t25277, t5985, t93021, t93035, t99066, t99070, t99074, t99078, t99086);
        let t106108 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2125(t18394, t7025, t27221, t62403, t18352, t1945, t807, t61639, t99062, t61725, t103329, t103347, t93049, t93067, t93073, t93088, t99100, t99103);
        let (t106111, t106116) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2126(t106005, t106020, t106028, t106055, t106067, t106078, t106092, t106108, t105974, t105976, t1580, t213, t225, t25322, t257, t6049, t92895, t92905, t98875, t98879, t98881, t98894, t98897, t98907, t98911, t99429);
        let (t106128, t106134) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2127(t29654, t686, t72, t25387, t25375, t29610, t27183, t27199, t92935, t93112, t93116, t93138, t93142, t98918, t98920, t99127, t99147, t99163, t99166);
    (t106111, t106116, t106128, t106134)
}
