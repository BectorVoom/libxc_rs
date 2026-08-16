//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1048 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3682;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3683;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3684;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1048(t68679: f64, t68704: f64, t68736: f64, t68773: f64, t68943: f64, t69031: f64, t69600: f64, t69606: f64, t5284: f64, t487: f64, t3565: f64, t6563: f64, t225: f64, t1234: f64, t1269: f64, t1285: f64, t1287: f64, t1288: f64, t12966: f64, t16776: f64, t17170: f64, t17307: f64, t17815: f64, t17934: f64, t1818: f64, t20721: f64, t20900: f64, t21082: f64, t21538: f64, t21565: f64, t3666: f64, t3670: f64, t3751: f64, t3759: f64, t3782: f64, t3783: f64, t3787: f64, t5216: f64, t5332: f64, t5443: f64, t5462: f64, t5463: f64, t5464: f64, t5466: f64, t59032: f64, t59241: f64, t6564: f64, t68674: f64, t12050: f64, t20956: f64, t1261: f64, t12879: f64, t247: f64, t6429: f64, t11262: f64, t1247: f64, t6624: f64, t21102: f64, t3704: f64, t17376: f64, t17524: f64, t17528: f64, t3140: f64, t3599: f64, t1042: f64, t17199: f64, t17204: f64, t17235: f64, t17558: f64, t21107: f64, t3591: f64, t3606: f64, t3613: f64, t5279: f64, t5302: f64, t5381: f64, t5391: f64, t58927: f64, t60834: f64, t65829: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69609, t69623, t69624, t69636) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3682(t68679, t68704, t68736, t68773, t68943, t69031, t69600, t69606, t5284, t487, t3565, t6563);
        let (t69637, t69652) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3683(t225, t69636, t1234, t1269, t1285, t1287, t1288, t12966, t16776, t17170, t17307, t17815, t17934, t1818, t20721, t20900, t21082, t21538, t21565, t3666, t3670, t3751, t3759, t3782, t3783, t3787, t487, t5216, t5332, t5443, t5462, t5463, t5464, t5466, t59032, t59241, t6564, t68674, t69609, t69624);
        let (t69655, t69661, t69668, t69674, t69680) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3684(t12050, t20956, t1261, t12879, t247, t6429, t11262, t1247, t6624, t21102, t3704, t17376, t17524);
        let (t69692, t69696) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3685(t17376, t17528, t3140, t6564, t3599, t1042, t1261, t17199, t17204, t17235, t17558, t21107, t3591, t3606, t3613, t5279, t5302, t5381, t5391, t58927, t60834, t65829, t69661, t69668, t69674, t69680);
    (t69609, t69623, t69624, t69636, t69637, t69652, t69655, t69692, t69696)
}
