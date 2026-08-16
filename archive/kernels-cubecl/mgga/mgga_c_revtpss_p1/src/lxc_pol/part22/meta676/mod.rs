//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2654;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta676<F: Float>(t20956: F, t21506: F, t3153: F, t6688: F, t5465: F, t12709: F, t12723: F, t12751: F, t12756: F, t1285: F, t17192: F, t17861: F, t17949: F, t17958: F, t1822: F, t21465: F, t21468: F, t21473: F, t21480: F, t21484: F, t21491: F, t21495: F, t21500: F, t3746: F, t3755: F, t5436: F, t5446: F, t5459: F, t5463: F, t5466: F, t5478: F, t5491: F, t6717: F, t6731: F, t5480: F, t1280: F, t20747: F, t5230: F, t5486: F, t21342: F, t489: F, t1248: F, t1287: F, t6695: F, t1774: F, t17821: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21507, t21512, t21513, t21516) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2654::<F>(t20956, t21506, t3153, t6688, t5465, t12709, t12723, t12751, t12756, t1285, t17192, t17861, t17949, t17958, t1822, t21465, t21468, t21473, t21480, t21484, t21491, t21495, t21500, t3746, t3755, t5436, t5446, t5459, t5463, t5466, t5478, t5491, t6717, t6731);
        let (t21518, t21521, t21524, t21527, t21535, t21538) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2655::<F>(t21512, t5480, t1280, t20747, t5230, t5486, t21342, t489, t1248, t1287, t6695, t1774, t17821);
    (t21507, t21512, t21513, t21516, t21518, t21521, t21524, t21527, t21535, t21538)
}
