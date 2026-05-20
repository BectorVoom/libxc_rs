//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta457 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1743;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1744;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1745;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1746;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1747;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1748;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1749;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1750;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1751;
use chunk9::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1752;
use chunk10::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1753;
use chunk11::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1754;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta457<F: Float>(t487: F, t5216: F, t1211: F, t16771: F, t16775: F, t1210: F, t1215: F, t12603: F, t1295: F, t18043: F, t18047: F, t18054: F, t18059: F, t18062: F, t1813: F, t1829: F, t3552: F, t3556: F, t3567: F, t3569: F, t3572: F, t3585: F, t5220: F, t5246: F, t5251: F, t5423: F, t1277: F, t1774: F, t3790: F, t1204: F, t1811: F, t16750: F, t1209: F, t5412: F, t1828: F, t3568: F, t1294: F, t5497: F, t3737: F, t17288: F, t12666: F, t12673: F, t1274: F, t1770: F, t1775: F, t3729: F, t3732: F, t3791: F, t5225: F, t5237: F, t5414: F, t5417: F, t5498: F, t18004: F, t18040: F, t1300: F, t16641: F, t16645: F, t16647: F, t16649: F, t16651: F, t16654: F, t16657: F, t16660: F, t16664: F, t16667: F, t16671: F, t16675: F, t16679: F, t16681: F, t16684: F, t16687: F, t16690: F, t16783: F, t198: F, t336: F, t3801: F, t5501: F, t12587: F, t1832: F, t1298: F, t16786: F, t16788: F, t16790: F, t16809: F, t16814: F, t16834: F, t16837: F, t16839: F, t16842: F, t16844: F, t16846: F, t16945: F, t17094: F, t17160: F, t17162: F, t17166: F, t17168: F, t3794: F, t3798: F, t5023: F, t5505: F, t33: F, t265: F, t502: F, t15083: F, t1113: F, t1304: F, t13312: F, t1469: F, t15093: F, t15094: F, t15096: F, t1587: F, t1711: F, t1837: F, t2258: F, t2838: F, t3351: F, t3805: F, t4186: F, t4560: F, t504: F, t5509: F, t57: F, t606: F, dens_threshold: F, rho1: F, zeta_threshold: F, t16630: F, t2371: F, t94: F, t118: F, t1310: F, t1315: F, t13425: F, t13426: F, t13429: F, t14310: F, t1519: F, t1843: F, t1847: F, t1911: F, t2320: F, t2322: F, t2331: F, t3821: F, t4151: F, t4246: F, t4248: F, t4254: F, t4257: F, t4293: F, t508: F, t511: F, t5517: F, t5787: F, t649: F, t671: F, t13547: F, t3: F, t1518: F, t2327: F, t116: F, t4292: F, t670: F, t5801: F, t117: F, t13514: F, param_d: F, t1459: F, t1461: F, t1916: F, t1918: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t1501: F, t2723: F, t4423: F, t1544: F, t890: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t18080 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1743::<F>(t487, t5216, t1211, t16771, t16775, t1210, t1215, t12603, t1295, t18043, t18047, t18054, t18059, t18062, t1813, t1829, t3552, t3556, t3567, t3569, t3572, t3585, t5220, t5246, t5251, t5423);
        let (t18084, t18087, t18090, t18097, t18103, t18108) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1744::<F>(t1277, t1774, t3790, t1204, t1811, t1211, t16750, t1209, t5412, t1828, t3568, t1294, t5497);
        let t18121 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1745::<F>(t18108, t3737, t17288, t487, t1204, t1210, t1215, t12666, t12673, t1274, t1295, t1770, t1775, t18084, t18087, t18090, t18097, t18103, t1829, t3556, t3567, t3729, t3732, t3791, t5225, t5237, t5414, t5417, t5498);
        let t18127 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1746::<F>(t18004, t18040, t18080, t18121, t1300, t16641, t16645, t16647, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16671, t16675, t16679, t16681, t16684, t16687, t16690, t16783, t198, t336);
        let t18138 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1747::<F>(t3801, t5501, t12587, t1832, t1298, t16786, t16788, t16790, t16809, t16814, t16834, t16837, t16839, t16842, t16844, t16846, t16945, t17094, t17160, t17162, t17166, t17168, t3794, t3798, t5023, t5505);
        let t18152 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1748::<F>(t33, t265, t502, t15083, t18127, t18138, t1113, t1304, t13312, t1469, t15093, t15094, t15096, t1587, t1711, t1837, t2258, t2838, t3351, t3805, t4186, t4560, t504, t5509, t57, t606, dens_threshold, rho1, zeta_threshold);
        let (t18153, t18163) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1749::<F>(t16630, t18152, t2371, t94);
        let t18176 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1750::<F>(t118, t1310, t1315, t13425, t13426, t13429, t14310, t1519, t18153, t18163, t1843, t1847, t1911, t2320, t2322, t2331, t3821, t4151, t4246, t4248, t4254, t4257, t4293, t508, t511, t5517, t5787, t649, t671);
        let (t18178, t18190, t18204, t18208, t18211, t18214) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1751::<F>(t13547, t18176, t3, t1518, t2327, t116, t4292, t670, t2371, t5801, t117, t13514, param_d);
        let t18217 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1752::<F>(t1459, t1461, t18190, t18204, t18208, t18211, t18214, t1916, t1918, t4158, t4162, t4165, t572, t573, t5795, t5802, t5805);
        let t18227 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1753::<F>(t1501, t670);
        let (t18632, t18875) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1754::<F>(t2723, t4423, t1544, t890);
    (t18153, t18163, t18178, t18190, t18204, t18208, t18211, t18214, t18217, t18227, t18632, t18875)
}
