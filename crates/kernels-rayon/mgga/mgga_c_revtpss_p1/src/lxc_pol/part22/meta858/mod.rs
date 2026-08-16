//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta858 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3005;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta858(t1565: f64, t40781: f64, t40488: f64, t4354: f64, t14862: f64, t9775: f64, t268: f64, t40452: f64, t4371: f64, t2662: f64, t40689: f64, t4353: f64, t10722: f64, t4345: f64, t40710: f64, t4349: f64, t14834: f64, t10716: f64, t14857: f64, t2475: f64, t4343: f64, t14832: f64, t2661: f64, t775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50370, t50372, t50374, t50377, t50381) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3005(t1565, t40781, t40488, t4354, t14862, t9775, t268, t40452, t4371, t2662, t40689, t4353);
        let (t50383, t50385, t50387, t50389, t50394) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3006(t10722, t4345, t40710, t4349, t14834, t9775, t10716, t14857, t2475, t4343, t14832, t2661, t775);
    (t50370, t50372, t50374, t50377, t50381, t50383, t50385, t50387, t50389, t50394)
}
