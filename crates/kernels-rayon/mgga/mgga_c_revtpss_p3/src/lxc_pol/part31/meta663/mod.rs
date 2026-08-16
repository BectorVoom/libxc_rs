//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta663 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2245;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2246;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2247;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2248;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2249;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2250;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2251;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2252;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2253;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta663(t1937: f64, t85360: f64, t18245: f64, t6993: f64, t1448: f64, t30122: f64, t25082: f64, t28197: f64, t105886: f64, t1312: f64, t1936: f64, t75439: f64, t7002: f64, t109150: f64, t109153: f64, t105866: f64, t108120: f64, t1518: f64, t21881: f64, t25805: f64, t28025: f64, t28030: f64, t33602: f64, t4292: f64, t5920: f64, t670: f64, t6985: f64, t97622: f64, t30138: f64, t13426: f64, t7741: f64, t18227: f64, t28042: f64, t4248: f64, t108710: f64, t93: f64, t30143: f64, t27123: f64, t28219: f64, t7889: f64, t2322: f64, t30004: f64, t5523: f64, t105850: f64, t109006: f64, t27833: f64, t7935: f64, t6922: f64, t28196: f64, t28067: f64, t98450: f64, t7897: f64, t8995: f64, t28199: f64, t27153: f64, t33651: f64, t109172: f64, t109176: f64, t109178: f64, t109180: f64, t109182: f64, t109194: f64, t30119: f64, t4254: f64, t569: f64, t651: f64, t7221: f64, t7883: f64, t108071: f64, t108114: f64, t108713: f64, t109030: f64, t109075: f64, t109129: f64, t109170: f64, t6941: f64, t7331: f64, t5795: f64, t7950: f64, t105818: f64, t105822: f64, t105826: f64, t105830: f64, t105834: f64, t105837: f64, t105839: f64, t105841: f64, t105843: f64, t1461: f64, t2040: f64, t22556: f64, t22568: f64, t30171: f64, t573: f64, t5805: f64, t6945: f64, t7324: f64, t7944: f64, param_d: f64, t7953: f64, t1916: f64, t28265: f64, t28277: f64, t572: f64, t28280: f64, t1459: f64, t30191: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t109196, t109198, t109202, t109204, t109222) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2245(t1937, t85360, t18245, t6993, t1448, t30122, t25082, t28197, t105886, t1312, t1936, t75439);
        let t109231 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2246(t1936, t85360, t18245, t7002, t109150, t109153, t105866, t108120, t109204, t109222, t1518, t21881, t25805, t28025, t28030, t33602, t4292, t5920, t670, t6985, t97622);
        let (t109233, t109235, t109237, t109239, t109241, t109244, t109246) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2247(t30138, t7002, t13426, t7741, t18227, t28042, t4248, t108710, t1936, t21881, t93, t30143);
        let t109258 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2248(t27123, t7741, t28219, t28042, t7889, t2322, t30004, t5523, t105850, t109006, t109233, t109235, t109237, t109239, t109241, t109244, t109246);
        let (t109262, t109266, t109268, t109271, t109274) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2249(t27833, t7935, t1448, t6922, t28196, t28197, t28067, t98450, t7897, t8995, t28199, t25082, t27153, t33651);
        let t109275 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2250(t109172, t109176, t109178, t109180, t109182, t109194, t109196, t109198, t109202, t109231, t109258, t109262, t109266, t109268, t109271, t109274, t2322, t30119, t4254, t4292, t569, t5920, t651, t7221, t7883);
        let (t109278, t109282, t109288) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2251(t108071, t108114, t108713, t109030, t109075, t109129, t109170, t109275, t6941, t7331, t5795, t7950);
        let t109289 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2252(t105818, t105822, t105826, t105830, t105834, t105837, t105839, t105841, t105843, t109278, t109282, t109288, t1461, t2040, t22556, t22568, t30171, t573, t5805, t6945, t7324, t7944, param_d);
        let (t109291, t109293, t109295, t109299, t109305, t109307) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2253(t5795, t7953, t1916, t28265, t28277, t1518, t572, t670, t7741, t28280, t1459, t30191);
    (t109278, t109289, t109291, t109293, t109295, t109299, t109305, t109307)
}
