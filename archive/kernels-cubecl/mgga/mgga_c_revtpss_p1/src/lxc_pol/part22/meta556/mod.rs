//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2382;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2383;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta556<F: Float>(t17845: F, t460: F, t12050: F, t13045: F, t3601: F, t17710: F, t13141: F, t487: F, t3603: F, t1234: F, t12717: F, t12751: F, t12756: F, t1285: F, t12966: F, t12975: F, t17188: F, t17192: F, t17808: F, t17811: F, t17815: F, t17818: F, t17822: F, t17826: F, t17829: F, t17834: F, t17837: F, t17840: F, t1818: F, t3666: F, t3670: F, t3755: F, t3756: F, t3767: F, t5443: F, t5452: F, t5463: F, t1284: F, t5216: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17846, t17847, t17848, t17849, t17852, t17853, t17854, t17855, t17856, t17859) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2382::<F>(t17845, t460, t12050, t13045, t3601, t17710, t13141, t487, t3603, t1234, t12717, t12751, t12756, t1285, t12966, t12975, t17188, t17192, t17808, t17811, t17815, t17818, t17822, t17826, t17829, t17834, t17837, t17840, t1818, t3666, t3670, t3755, t3756, t3767, t5443, t5452, t5463);
        let t17861 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2383::<F>(t1284, t5216);
    (t17846, t17847, t17848, t17849, t17852, t17853, t17854, t17855, t17856, t17859, t17861)
}
