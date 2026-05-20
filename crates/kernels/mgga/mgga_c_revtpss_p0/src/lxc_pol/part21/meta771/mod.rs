//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta771 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2738;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2739;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2740;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta771<F: Float>(t1565: F, t40781: F, t40488: F, t4354: F, t14862: F, t9775: F, t268: F, t40452: F, t4371: F, t2662: F, t40689: F, t4353: F, t10722: F, t4345: F, t40710: F, t4349: F, t14834: F, t10716: F, t14857: F, t2475: F, t4343: F, t14832: F, t2661: F, t775: F, t10696: F, t1544: F, t2394: F, t40409: F, t40411: F, t40413: F, t40421: F, t40425: F, t40429: F, t50151: F, t828: F, t851: F, t855: F, t14668: F, t14923: F, t124: F, t4423: F, t14686: F, t14931: F, t4366: F, t2645: F, t2722: F, t1558: F, t231: F, t40406: F, t685: F, t72: F, t826: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50370, t50372, t50375, t50377, t50381) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2738::<F>(t1565, t40781, t40488, t4354, t14862, t9775, t268, t40452, t4371, t2662, t40689, t4353);
        let (t50383, t50385, t50387, t50390, t50391, t50394) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2739::<F>(t10722, t4345, t40710, t4349, t14834, t9775, t10716, t14857, t2475, t4343, t14832, t2661, t775);
        let (t50396, t50408) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2740::<F>(t10696, t1544, t14832, t2394, t2661, t40409, t40411, t40413, t40421, t40425, t40429, t50151, t50370, t50372, t50375, t50377, t50381, t50383, t50385, t50387, t50390, t50394, t828, t851, t855);
        let (t50409, t50412, t50415, t50418, t50423, t50436) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2741::<F>(t14668, t14923, t124, t4423, t14686, t14931, t4366, t1544, t2645, t2722, t1558, t231, t40406, t685, t72, t826);
    (t50391, t50396, t50408, t50409, t50412, t50415, t50418, t50423, t50436)
}
