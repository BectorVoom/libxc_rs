//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2294;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2295;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2296;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2297;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta583(t487: f64, t5216: f64, t1211: f64, t16771: f64, t16775: f64, t1210: f64, t1215: f64, t12603: f64, t1295: f64, t18043: f64, t18047: f64, t18054: f64, t18059: f64, t18062: f64, t1813: f64, t1829: f64, t3552: f64, t3556: f64, t3567: f64, t3569: f64, t3572: f64, t3585: f64, t5220: f64, t5246: f64, t5251: f64, t5423: f64, t1277: f64, t1774: f64, t3790: f64, t1204: f64, t1811: f64, t16750: f64, t1209: f64, t5412: f64, t1828: f64, t3568: f64, t1294: f64, t5497: f64, t3737: f64, t17288: f64, t12666: f64, t12673: f64, t1274: f64, t1770: f64, t1775: f64, t3729: f64, t3732: f64, t3791: f64, t5225: f64, t5237: f64, t5414: f64, t5417: f64, t5498: f64, t18004: f64, t18040: f64, t1300: f64, t16641: f64, t16645: f64, t16647: f64, t16649: f64, t16651: f64, t16654: f64, t16657: f64, t16660: f64, t16664: f64, t16667: f64, t16671: f64, t16675: f64, t16679: f64, t16681: f64, t16684: f64, t16687: f64, t16690: f64, t16783: f64, t198: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18065, t18070, t18073, t18080) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2294(t487, t5216, t1211, t16771, t16775, t1210, t1215, t12603, t1295, t18043, t18047, t18054, t18059, t18062, t1813, t1829, t3552, t3556, t3567, t3569, t3572, t3585, t5220, t5246, t5251, t5423);
        let (t18084, t18087, t18090, t18097, t18103, t18108) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2295(t1277, t1774, t3790, t1204, t1811, t1211, t16750, t1209, t5412, t1828, t3568, t1294, t5497);
        let (t18109, t18114, t18121) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2296(t18108, t3737, t17288, t487, t1204, t1210, t1215, t12666, t12673, t1274, t1295, t1770, t1775, t18084, t18087, t18090, t18097, t18103, t1829, t3556, t3567, t3729, t3732, t3791, t5225, t5237, t5414, t5417, t5498);
        let (t18123, t18127) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2297(t18004, t18040, t18080, t18121, t1300, t16641, t16645, t16647, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16671, t16675, t16679, t16681, t16684, t16687, t16690, t16783, t198, t336);
    (t18065, t18070, t18073, t18084, t18087, t18090, t18097, t18103, t18109, t18114, t18123, t18127)
}
