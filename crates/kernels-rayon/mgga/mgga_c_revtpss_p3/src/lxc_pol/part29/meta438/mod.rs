//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta438 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1638;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1639;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1640;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1641;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1642;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1643;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1644;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1645;
use chunk8::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1646;
use chunk9::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1647;
use chunk10::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1648;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta438(t487: f64, t5216: f64, t1211: f64, t16771: f64, t16775: f64, t1210: f64, t1215: f64, t12603: f64, t1295: f64, t18043: f64, t18047: f64, t18054: f64, t18059: f64, t18062: f64, t1813: f64, t1829: f64, t3552: f64, t3556: f64, t3567: f64, t3569: f64, t3572: f64, t3585: f64, t5220: f64, t5246: f64, t5251: f64, t5423: f64, t1277: f64, t1774: f64, t3790: f64, t1204: f64, t1811: f64, t16750: f64, t1209: f64, t5412: f64, t1828: f64, t3568: f64, t1294: f64, t5497: f64, t3737: f64, t17288: f64, t12666: f64, t12673: f64, t1274: f64, t1770: f64, t1775: f64, t3729: f64, t3732: f64, t3791: f64, t5225: f64, t5237: f64, t5414: f64, t5417: f64, t5498: f64, t18004: f64, t18040: f64, t1300: f64, t16641: f64, t16645: f64, t16647: f64, t16649: f64, t16651: f64, t16654: f64, t16657: f64, t16660: f64, t16664: f64, t16667: f64, t16671: f64, t16675: f64, t16679: f64, t16681: f64, t16684: f64, t16687: f64, t16690: f64, t16783: f64, t198: f64, t336: f64, t3801: f64, t5501: f64, t12587: f64, t1832: f64, t1298: f64, t16786: f64, t16788: f64, t16790: f64, t16809: f64, t16814: f64, t16834: f64, t16837: f64, t16839: f64, t16842: f64, t16844: f64, t16846: f64, t16945: f64, t17094: f64, t17160: f64, t17162: f64, t17166: f64, t17168: f64, t3794: f64, t3798: f64, t5023: f64, t5505: f64, t33: f64, t265: f64, t502: f64, t15083: f64, t1113: f64, t1304: f64, t13312: f64, t1469: f64, t15093: f64, t15094: f64, t15096: f64, t1587: f64, t1711: f64, t1837: f64, t2258: f64, t2838: f64, t3351: f64, t3805: f64, t4186: f64, t4560: f64, t504: f64, t5509: f64, t57: f64, t606: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t16630: f64, t2371: f64, t94: f64, t118: f64, t1310: f64, t1315: f64, t13425: f64, t13426: f64, t13429: f64, t14310: f64, t1519: f64, t1843: f64, t1847: f64, t1911: f64, t2320: f64, t2322: f64, t2331: f64, t3821: f64, t4151: f64, t4246: f64, t4248: f64, t4254: f64, t4257: f64, t4293: f64, t508: f64, t511: f64, t5517: f64, t5787: f64, t649: f64, t671: f64, t13547: f64, t3: f64, t1518: f64, t2327: f64, t116: f64, t4292: f64, t670: f64, t5801: f64, t117: f64, t13514: f64, param_d: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t4158: f64, t4162: f64, t4165: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t1501: f64, t2723: f64, t4423: f64, t1544: f64, t890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t18080 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1638(t487, t5216, t1211, t16771, t16775, t1210, t1215, t12603, t1295, t18043, t18047, t18054, t18059, t18062, t1813, t1829, t3552, t3556, t3567, t3569, t3572, t3585, t5220, t5246, t5251, t5423);
        let (t18084, t18087, t18090, t18097, t18103, t18108) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1639(t1277, t1774, t3790, t1204, t1811, t1211, t16750, t1209, t5412, t1828, t3568, t1294, t5497);
        let t18121 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1640(t18108, t3737, t17288, t487, t1204, t1210, t1215, t12666, t12673, t1274, t1295, t1770, t1775, t18084, t18087, t18090, t18097, t18103, t1829, t3556, t3567, t3729, t3732, t3791, t5225, t5237, t5414, t5417, t5498);
        let t18127 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1641(t18004, t18040, t18080, t18121, t1300, t16641, t16645, t16647, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16671, t16675, t16679, t16681, t16684, t16687, t16690, t16783, t198, t336);
        let t18138 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1642(t3801, t5501, t12587, t1832, t1298, t16786, t16788, t16790, t16809, t16814, t16834, t16837, t16839, t16842, t16844, t16846, t16945, t17094, t17160, t17162, t17166, t17168, t3794, t3798, t5023, t5505);
        let t18152 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1643(t33, t265, t502, t15083, t18127, t18138, t1113, t1304, t13312, t1469, t15093, t15094, t15096, t1587, t1711, t1837, t2258, t2838, t3351, t3805, t4186, t4560, t504, t5509, t57, t606, dens_threshold, rho1, zeta_threshold);
        let (t18153, t18163, t18176) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1644(t16630, t18152, t2371, t94, t118, t1310, t1315, t13425, t13426, t13429, t14310, t1519, t1843, t1847, t1911, t2320, t2322, t2331, t3821, t4151, t4246, t4248, t4254, t4257, t4293, t508, t511, t5517, t5787, t649, t671);
        let (t18178, t18190, t18204, t18208, t18211, t18214) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1645(t13547, t18176, t3, t1518, t2327, t116, t4292, t670, t2371, t5801, t117, t13514, param_d);
        let t18217 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1646(t1459, t1461, t18190, t18204, t18208, t18211, t18214, t1916, t1918, t4158, t4162, t4165, t572, t573, t5795, t5802, t5805);
        let t18227 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1647(t1501, t670);
        let (t18632, t18875) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1648(t2723, t4423, t1544, t890);
    (t18153, t18163, t18178, t18190, t18204, t18208, t18211, t18214, t18217, t18227, t18632, t18875)
}
