//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2047;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta527(t1770: f64, t5462: f64, t12050: f64, t1248: f64, t471: f64, t20956: f64, t3153: f64, t6688: f64, t5465: f64, t12709: f64, t12723: f64, t12751: f64, t12756: f64, t1285: f64, t17192: f64, t17861: f64, t17949: f64, t17958: f64, t1822: f64, t21465: f64, t21468: f64, t21473: f64, t21480: f64, t21484: f64, t21491: f64, t21495: f64, t3746: f64, t3755: f64, t5436: f64, t5446: f64, t5459: f64, t5463: f64, t5466: f64, t5478: f64, t5491: f64, t6717: f64, t6731: f64, t5480: f64, t1280: f64, t20747: f64, t5230: f64, t5486: f64, t21342: f64, t489: f64, t1287: f64, t6695: f64, t1774: f64, t17821: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21500, t21506, t21507, t21512, t21513, t21516) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2047(t1770, t5462, t12050, t1248, t471, t20956, t3153, t6688, t5465, t12709, t12723, t12751, t12756, t1285, t17192, t17861, t17949, t17958, t1822, t21465, t21468, t21473, t21480, t21484, t21491, t21495, t3746, t3755, t5436, t5446, t5459, t5463, t5466, t5478, t5491, t6717, t6731);
        let (t21518, t21521, t21524, t21527, t21535, t21538) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2048(t21512, t5480, t1280, t20747, t5230, t5486, t21342, t489, t1248, t1287, t6695, t1774, t17821);
    (t21500, t21506, t21507, t21512, t21513, t21516, t21518, t21521, t21524, t21527, t21535, t21538)
}
