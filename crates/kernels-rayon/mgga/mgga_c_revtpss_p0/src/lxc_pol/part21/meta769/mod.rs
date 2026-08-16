//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta769 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2723;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2724;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2725;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta769(t2609: f64, t4395: f64, t14341: f64, t2398: f64, t40145: f64, t11084: f64, t15078: f64, t40141: f64, t4433: f64, t4541: f64, t50080: f64, t50085: f64, t50091: f64, t50093: f64, t50095: f64, t50096: f64, t45: f64, t40148: f64, t13312: f64, t706: f64, t750: f64, t40150: f64, t10326: f64, t10356: f64, t11231: f64, t14447: f64, t1490: f64, t2251: f64, t2258: f64, t4227: f64, t4230: f64, t4328: f64, t49889: f64, t606: f64, t766: f64, t80: f64, zeta_threshold: f64, t57: f64, t14458: f64, t1491: f64, t4232: f64, t4235: f64, t4335: f64, t770: f64, t83: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50098, t50100, t50101, t50102) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2723(t2609, t4395, t14341, t2398, t40145, t11084, t15078, t40141, t4433, t4541, t50080, t50085, t50091, t50093, t50095, t50096);
        let (t50106, t50114, t50115, t50132) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2724(t45, t40148, t13312, t706, t750, t40150, t10326, t10356, t11231, t14447, t1490, t2251, t2258, t4227, t4230, t4328, t49889, t606, t766, t80, zeta_threshold);
        let t50149 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2725(t57, t10326, t10356, t11231, t13312, t14458, t1491, t2251, t2258, t4232, t4235, t4335, t49889, t606, t770, t83, zeta_threshold);
    (t50098, t50100, t50101, t50102, t50106, t50114, t50115, t50132, t50149)
}
