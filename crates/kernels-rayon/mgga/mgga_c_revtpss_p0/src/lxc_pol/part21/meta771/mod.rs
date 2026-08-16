//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta771 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2738;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2739;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2740;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta771(t1565: f64, t40781: f64, t40488: f64, t4354: f64, t14862: f64, t9775: f64, t268: f64, t40452: f64, t4371: f64, t2662: f64, t40689: f64, t4353: f64, t10722: f64, t4345: f64, t40710: f64, t4349: f64, t14834: f64, t10716: f64, t14857: f64, t2475: f64, t4343: f64, t14832: f64, t2661: f64, t775: f64, t10696: f64, t1544: f64, t2394: f64, t40409: f64, t40411: f64, t40413: f64, t40421: f64, t40425: f64, t40429: f64, t50151: f64, t828: f64, t851: f64, t855: f64, t14668: f64, t14923: f64, t124: f64, t4423: f64, t14686: f64, t14931: f64, t4366: f64, t2645: f64, t2722: f64, t1558: f64, t231: f64, t40406: f64, t685: f64, t72: f64, t826: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50370, t50372, t50375, t50377, t50381) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2738(t1565, t40781, t40488, t4354, t14862, t9775, t268, t40452, t4371, t2662, t40689, t4353);
        let (t50383, t50385, t50387, t50390, t50391, t50394) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2739(t10722, t4345, t40710, t4349, t14834, t9775, t10716, t14857, t2475, t4343, t14832, t2661, t775);
        let (t50396, t50408) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2740(t10696, t1544, t14832, t2394, t2661, t40409, t40411, t40413, t40421, t40425, t40429, t50151, t50370, t50372, t50375, t50377, t50381, t50383, t50385, t50387, t50390, t50394, t828, t851, t855);
        let (t50409, t50412, t50415, t50418, t50423, t50436) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2741(t14668, t14923, t124, t4423, t14686, t14931, t4366, t1544, t2645, t2722, t1558, t231, t40406, t685, t72, t826);
    (t50391, t50396, t50408, t50409, t50412, t50415, t50418, t50423, t50436)
}
