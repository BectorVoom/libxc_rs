//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2382;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2383;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta556(t17845: f64, t460: f64, t12050: f64, t13045: f64, t3601: f64, t17710: f64, t13141: f64, t487: f64, t3603: f64, t1234: f64, t12717: f64, t12751: f64, t12756: f64, t1285: f64, t12966: f64, t12975: f64, t17188: f64, t17192: f64, t17808: f64, t17811: f64, t17815: f64, t17818: f64, t17822: f64, t17826: f64, t17829: f64, t17834: f64, t17837: f64, t17840: f64, t1818: f64, t3666: f64, t3670: f64, t3755: f64, t3756: f64, t3767: f64, t5443: f64, t5452: f64, t5463: f64, t1284: f64, t5216: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17846, t17847, t17848, t17849, t17852, t17853, t17854, t17855, t17856, t17859) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2382(t17845, t460, t12050, t13045, t3601, t17710, t13141, t487, t3603, t1234, t12717, t12751, t12756, t1285, t12966, t12975, t17188, t17192, t17808, t17811, t17815, t17818, t17822, t17826, t17829, t17834, t17837, t17840, t1818, t3666, t3670, t3755, t3756, t3767, t5443, t5452, t5463);
        let t17861 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2383(t1284, t5216);
    (t17846, t17847, t17848, t17849, t17852, t17853, t17854, t17855, t17856, t17859, t17861)
}
