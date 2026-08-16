//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1720;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1721;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1722;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta518(t29009: f64, t29054: f64, t858: f64, t2053: f64, t2718: f64, t5657: f64, t218: f64, t29040: f64, t1528: f64, t17090: f64, t2054: f64, t25036: f64, t25049: f64, t259: f64, t26713: f64, t28265: f64, t28269: f64, t28274: f64, t28278: f64, t28289: f64, t28296: f64, t28300: f64, t4147: f64, t4268: f64, t5637: f64, t7087: f64, t7830: f64, t855: f64, t1527: f64, t7841: f64, t10110: f64, t5636: f64, t2047: f64, t5558: f64, t1492: f64, t7823: f64, t17052: f64, t17092: f64, t24291: f64, t24318: f64, t24321: f64, t25206: f64, t25209: f64, t25211: f64, t25346: f64, t26700: f64, t28440: f64, t5658: f64, t7842: f64, t870: f64, t1408: f64, t1877: f64, t2057: f64, t24191: f64, t24344: f64, t25: f64, t2522: f64, t26744: f64, t28249: f64, t28252: f64, t28256: f64, t28456: f64, t28459: f64, t28462: f64, t28972: f64, t4314: f64, t5397: f64, t7114: f64, t7475: f64, t7545: f64, t7845: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29055, t29056, t29060, t29071, t29075) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1720(t29009, t29054, t858, t2053, t2718, t5657, t218, t29040, t1528, t17090, t2054, t25036, t25049, t259, t26713, t28265, t28269, t28274, t28278, t28289, t28296, t28300, t4147, t4268, t5637, t7087, t7830, t855);
        let (t29080, t29091, t29095, t29099, t29104) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1721(t1527, t7841, t2718, t10110, t2053, t5636, t2047, t5558, t1492, t7823, t1528, t17052, t17092, t2054, t24291, t24318, t24321, t25206, t25209, t25211, t25346, t259, t26700, t28440, t4147, t4268, t5658, t7087, t7842, t855);
        let (t29105, t29106) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1722(t29075, t29104, t870);
        let t29124 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1723(t1408, t1877, t2057, t24191, t24344, t25, t2522, t26744, t28249, t28252, t28256, t28456, t28459, t28462, t28972, t29106, t4314, t5397, t7114, t7475, t7545, t7845);
    (t29055, t29056, t29060, t29071, t29080, t29091, t29095, t29099, t29105, t29106, t29124)
}
