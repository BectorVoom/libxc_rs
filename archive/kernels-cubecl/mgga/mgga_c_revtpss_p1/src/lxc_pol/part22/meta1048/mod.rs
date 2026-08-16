//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1048 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3682;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3683;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3684;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1048<F: Float>(t68679: F, t68704: F, t68736: F, t68773: F, t68943: F, t69031: F, t69600: F, t69606: F, t5284: F, t487: F, t3565: F, t6563: F, t225: F, t1234: F, t1269: F, t1285: F, t1287: F, t1288: F, t12966: F, t16776: F, t17170: F, t17307: F, t17815: F, t17934: F, t1818: F, t20721: F, t20900: F, t21082: F, t21538: F, t21565: F, t3666: F, t3670: F, t3751: F, t3759: F, t3782: F, t3783: F, t3787: F, t5216: F, t5332: F, t5443: F, t5462: F, t5463: F, t5464: F, t5466: F, t59032: F, t59241: F, t6564: F, t68674: F, t12050: F, t20956: F, t1261: F, t12879: F, t247: F, t6429: F, t11262: F, t1247: F, t6624: F, t21102: F, t3704: F, t17376: F, t17524: F, t17528: F, t3140: F, t3599: F, t1042: F, t17199: F, t17204: F, t17235: F, t17558: F, t21107: F, t3591: F, t3606: F, t3613: F, t5279: F, t5302: F, t5381: F, t5391: F, t58927: F, t60834: F, t65829: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t69609, t69623, t69624, t69636) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3682::<F>(t68679, t68704, t68736, t68773, t68943, t69031, t69600, t69606, t5284, t487, t3565, t6563);
        let (t69637, t69652) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3683::<F>(t225, t69636, t1234, t1269, t1285, t1287, t1288, t12966, t16776, t17170, t17307, t17815, t17934, t1818, t20721, t20900, t21082, t21538, t21565, t3666, t3670, t3751, t3759, t3782, t3783, t3787, t487, t5216, t5332, t5443, t5462, t5463, t5464, t5466, t59032, t59241, t6564, t68674, t69609, t69624);
        let (t69655, t69661, t69668, t69674, t69680) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3684::<F>(t12050, t20956, t1261, t12879, t247, t6429, t11262, t1247, t6624, t21102, t3704, t17376, t17524);
        let (t69692, t69696) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3685::<F>(t17376, t17528, t3140, t6564, t3599, t1042, t1261, t17199, t17204, t17235, t17558, t21107, t3591, t3606, t3613, t5279, t5302, t5381, t5391, t58927, t60834, t65829, t69661, t69668, t69674, t69680);
    (t69609, t69623, t69624, t69636, t69637, t69652, t69655, t69692, t69696)
}
