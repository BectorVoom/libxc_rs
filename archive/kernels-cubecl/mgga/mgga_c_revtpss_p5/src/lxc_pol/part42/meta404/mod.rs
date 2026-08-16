//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1396;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1397;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1398;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1399;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta404<F: Float>(t1284: F, t6564: F, t6688: F, t73: F, t5458: F, t1287: F, t21257: F, t1811: F, t3766: F, t460: F, t3781: F, t21040: F, t12702: F, t12717: F, t12744: F, t1285: F, t1288: F, t17307: F, t17958: F, t21416: F, t21427: F, t21430: F, t21436: F, t3666: F, t3670: F, t3755: F, t3767: F, t3782: F, t5326: F, t5436: F, t5443: F, t5446: F, t5466: F, t5470: F, t5481: F, t5487: F, t6720: F, t6727: F, t6738: F, t20800: F, t5465: F, t5480: F, t3302: F, t471: F, t1214: F, t20795: F, t21298: F, t5464: F, t21164: F, t20900: F, t487: F, t1770: F, t5462: F, t12050: F, t1248: F, t20956: F, t3153: F, t12709: F, t12723: F, t12751: F, t12756: F, t17192: F, t17861: F, t17949: F, t1822: F, t3746: F, t5459: F, t5463: F, t5478: F, t5491: F, t6717: F, t6731: F, t1280: F, t20747: F, t5230: F, t5486: F, t21342: F, t489: F, t6695: F, t1774: F, t17821: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t21439, t21443, t21448, t21452, t21456, t21459) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1396::<F>(t1284, t6564, t6688, t73, t5458, t1287, t21257, t1811, t3766, t460, t3781, t21040);
        let t21464 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1397::<F>(t12702, t12717, t12744, t1285, t1288, t17307, t17958, t21416, t21427, t21430, t21436, t21439, t21443, t21448, t21452, t21456, t21459, t3666, t3670, t3755, t3767, t3782, t5326, t5436, t5443, t5446, t5466, t5470, t5481, t5487, t6720, t6727, t6738);
        let (t21465, t21468, t21471, t21473, t21480, t21484, t21491, t21495) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1398::<F>(t20800, t5465, t5480, t3302, t471, t1214, t20795, t1287, t21298, t5464, t21164, t20900, t487);
        let (t21512, t21516) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1399::<F>(t1770, t5462, t12050, t1248, t471, t20956, t3153, t6688, t5465, t12709, t12723, t12751, t12756, t1285, t17192, t17861, t17949, t17958, t1822, t21465, t21468, t21473, t21480, t21484, t21491, t21495, t3746, t3755, t5436, t5446, t5459, t5463, t5466, t5478, t5491, t6717, t6731);
        let (t21518, t21521, t21524, t21527, t21535, t21538) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1400::<F>(t21512, t5480, t1280, t20747, t5230, t5486, t21342, t489, t1248, t1287, t6695, t1774, t17821);
    (t21464, t21471, t21516, t21518, t21521, t21524, t21527, t21535, t21538)
}
