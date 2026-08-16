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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta663<F: Float>(t1937: F, t85360: F, t18245: F, t6993: F, t1448: F, t30122: F, t25082: F, t28197: F, t105886: F, t1312: F, t1936: F, t75439: F, t7002: F, t109150: F, t109153: F, t105866: F, t108120: F, t1518: F, t21881: F, t25805: F, t28025: F, t28030: F, t33602: F, t4292: F, t5920: F, t670: F, t6985: F, t97622: F, t30138: F, t13426: F, t7741: F, t18227: F, t28042: F, t4248: F, t108710: F, t93: F, t30143: F, t27123: F, t28219: F, t7889: F, t2322: F, t30004: F, t5523: F, t105850: F, t109006: F, t27833: F, t7935: F, t6922: F, t28196: F, t28067: F, t98450: F, t7897: F, t8995: F, t28199: F, t27153: F, t33651: F, t109172: F, t109176: F, t109178: F, t109180: F, t109182: F, t109194: F, t30119: F, t4254: F, t569: F, t651: F, t7221: F, t7883: F, t108071: F, t108114: F, t108713: F, t109030: F, t109075: F, t109129: F, t109170: F, t6941: F, t7331: F, t5795: F, t7950: F, t105818: F, t105822: F, t105826: F, t105830: F, t105834: F, t105837: F, t105839: F, t105841: F, t105843: F, t1461: F, t2040: F, t22556: F, t22568: F, t30171: F, t573: F, t5805: F, t6945: F, t7324: F, t7944: F, param_d: F, t7953: F, t1916: F, t28265: F, t28277: F, t572: F, t28280: F, t1459: F, t30191: F) -> (F, F, F, F, F, F, F, F) {
        let (t109196, t109198, t109202, t109204, t109222) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2245::<F>(t1937, t85360, t18245, t6993, t1448, t30122, t25082, t28197, t105886, t1312, t1936, t75439);
        let t109231 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2246::<F>(t1936, t85360, t18245, t7002, t109150, t109153, t105866, t108120, t109204, t109222, t1518, t21881, t25805, t28025, t28030, t33602, t4292, t5920, t670, t6985, t97622);
        let (t109233, t109235, t109237, t109239, t109241, t109244, t109246) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2247::<F>(t30138, t7002, t13426, t7741, t18227, t28042, t4248, t108710, t1936, t21881, t93, t30143);
        let t109258 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2248::<F>(t27123, t7741, t28219, t28042, t7889, t2322, t30004, t5523, t105850, t109006, t109233, t109235, t109237, t109239, t109241, t109244, t109246);
        let (t109262, t109266, t109268, t109271, t109274) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2249::<F>(t27833, t7935, t1448, t6922, t28196, t28197, t28067, t98450, t7897, t8995, t28199, t25082, t27153, t33651);
        let t109275 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2250::<F>(t109172, t109176, t109178, t109180, t109182, t109194, t109196, t109198, t109202, t109231, t109258, t109262, t109266, t109268, t109271, t109274, t2322, t30119, t4254, t4292, t569, t5920, t651, t7221, t7883);
        let (t109278, t109282, t109288) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2251::<F>(t108071, t108114, t108713, t109030, t109075, t109129, t109170, t109275, t6941, t7331, t5795, t7950);
        let t109289 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2252::<F>(t105818, t105822, t105826, t105830, t105834, t105837, t105839, t105841, t105843, t109278, t109282, t109288, t1461, t2040, t22556, t22568, t30171, t573, t5805, t6945, t7324, t7944, param_d);
        let (t109291, t109293, t109295, t109299, t109305, t109307) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2253::<F>(t5795, t7953, t1916, t28265, t28277, t1518, t572, t670, t7741, t28280, t1459, t30191);
    (t109278, t109289, t109291, t109293, t109295, t109299, t109305, t109307)
}
