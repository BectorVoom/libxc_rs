//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1669;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1670;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1671;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1672;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1673;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta458(t1284: f64, t6564: f64, t6688: f64, t73: f64, t5458: f64, t1287: f64, t21257: f64, t1811: f64, t3766: f64, t460: f64, t3781: f64, t21040: f64, t12702: f64, t12717: f64, t12744: f64, t1285: f64, t1288: f64, t17307: f64, t17958: f64, t21416: f64, t21427: f64, t21430: f64, t21436: f64, t3666: f64, t3670: f64, t3755: f64, t3767: f64, t3782: f64, t5326: f64, t5436: f64, t5443: f64, t5446: f64, t5466: f64, t5470: f64, t5481: f64, t5487: f64, t6720: f64, t6727: f64, t6738: f64, t20800: f64, t5465: f64, t5480: f64, t3302: f64, t471: f64, t1214: f64, t20795: f64, t21298: f64, t5464: f64, t21164: f64, t20900: f64, t487: f64, t1770: f64, t5462: f64, t12050: f64, t1248: f64, t20956: f64, t3153: f64, t12709: f64, t12723: f64, t12751: f64, t12756: f64, t17192: f64, t17861: f64, t17949: f64, t1822: f64, t3746: f64, t5459: f64, t5463: f64, t5478: f64, t5491: f64, t6717: f64, t6731: f64, t1280: f64, t20747: f64, t5230: f64, t5486: f64, t21342: f64, t489: f64, t6695: f64, t1774: f64, t17821: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21439, t21443, t21448, t21452, t21456, t21459) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1669(t1284, t6564, t6688, t73, t5458, t1287, t21257, t1811, t3766, t460, t3781, t21040);
        let t21464 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1670(t12702, t12717, t12744, t1285, t1288, t17307, t17958, t21416, t21427, t21430, t21436, t21439, t21443, t21448, t21452, t21456, t21459, t3666, t3670, t3755, t3767, t3782, t5326, t5436, t5443, t5446, t5466, t5470, t5481, t5487, t6720, t6727, t6738);
        let (t21465, t21468, t21471, t21473, t21480, t21484, t21491, t21495) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1671(t20800, t5465, t5480, t3302, t471, t1214, t20795, t1287, t21298, t5464, t21164, t20900, t487);
        let (t21512, t21516) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1672(t1770, t5462, t12050, t1248, t471, t20956, t3153, t6688, t5465, t12709, t12723, t12751, t12756, t1285, t17192, t17861, t17949, t17958, t1822, t21465, t21468, t21473, t21480, t21484, t21491, t21495, t3746, t3755, t5436, t5446, t5459, t5463, t5466, t5478, t5491, t6717, t6731);
        let (t21518, t21521, t21524, t21527, t21535, t21538) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1673(t21512, t5480, t1280, t20747, t5230, t5486, t21342, t489, t1248, t1287, t6695, t1774, t17821);
    (t21464, t21471, t21516, t21518, t21521, t21524, t21527, t21535, t21538)
}
