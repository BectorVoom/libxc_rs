//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1904;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1905;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta637(t28192: f64, t80727: f64, t1307: f64, t1377: f64, t22633: f64, t22635: f64, t6460: f64, t1842: f64, t26331: f64, t26337: f64, t26189: f64, t26193: f64, t6888: f64, t22892: f64, t7691: f64, t90544: f64, t1835: f64, t254: f64, t28200: f64, t6883: f64, t90739: f64, t1845: f64, t5187: f64, t191: f64, t192: f64, t19537: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97664, t97705, t97724, t97729) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1904(t28192, t80727, t1307, t1377, t22633, t22635, t6460, t1842, t26331, t26337, t26189, t26193, t6888);
        let (t97732, t97740, t97750, t97766, t97789, t97804) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1905(t22892, t7691, t90544, t1835, t254, t28200, t6883, t6888, t90739, t1845, t5187, t191, t192, t19537);
    (t97664, t97705, t97724, t97729, t97732, t97740, t97750, t97766, t97789, t97804)
}
