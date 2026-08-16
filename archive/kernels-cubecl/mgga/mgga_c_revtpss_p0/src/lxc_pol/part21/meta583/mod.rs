//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2294;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2295;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2296;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2297;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta583<F: Float>(t487: F, t5216: F, t1211: F, t16771: F, t16775: F, t1210: F, t1215: F, t12603: F, t1295: F, t18043: F, t18047: F, t18054: F, t18059: F, t18062: F, t1813: F, t1829: F, t3552: F, t3556: F, t3567: F, t3569: F, t3572: F, t3585: F, t5220: F, t5246: F, t5251: F, t5423: F, t1277: F, t1774: F, t3790: F, t1204: F, t1811: F, t16750: F, t1209: F, t5412: F, t1828: F, t3568: F, t1294: F, t5497: F, t3737: F, t17288: F, t12666: F, t12673: F, t1274: F, t1770: F, t1775: F, t3729: F, t3732: F, t3791: F, t5225: F, t5237: F, t5414: F, t5417: F, t5498: F, t18004: F, t18040: F, t1300: F, t16641: F, t16645: F, t16647: F, t16649: F, t16651: F, t16654: F, t16657: F, t16660: F, t16664: F, t16667: F, t16671: F, t16675: F, t16679: F, t16681: F, t16684: F, t16687: F, t16690: F, t16783: F, t198: F, t336: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18065, t18070, t18073, t18080) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2294::<F>(t487, t5216, t1211, t16771, t16775, t1210, t1215, t12603, t1295, t18043, t18047, t18054, t18059, t18062, t1813, t1829, t3552, t3556, t3567, t3569, t3572, t3585, t5220, t5246, t5251, t5423);
        let (t18084, t18087, t18090, t18097, t18103, t18108) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2295::<F>(t1277, t1774, t3790, t1204, t1811, t1211, t16750, t1209, t5412, t1828, t3568, t1294, t5497);
        let (t18109, t18114, t18121) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2296::<F>(t18108, t3737, t17288, t487, t1204, t1210, t1215, t12666, t12673, t1274, t1295, t1770, t1775, t18084, t18087, t18090, t18097, t18103, t1829, t3556, t3567, t3729, t3732, t3791, t5225, t5237, t5414, t5417, t5498);
        let (t18123, t18127) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2297::<F>(t18004, t18040, t18080, t18121, t1300, t16641, t16645, t16647, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16671, t16675, t16679, t16681, t16684, t16687, t16690, t16783, t198, t336);
    (t18065, t18070, t18073, t18084, t18087, t18090, t18097, t18103, t18109, t18114, t18123, t18127)
}
