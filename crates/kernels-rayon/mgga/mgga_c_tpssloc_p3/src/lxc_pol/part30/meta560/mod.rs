//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1920;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta560(t1902: f64, t5558: f64, t25224: f64, t7479: f64, t6552: f64, t23195: f64, t5636: f64, t6553: f64, t1880: f64, t5527: f64, t6554: f64, t23035: f64, t1528: f64, t17052: f64, t17092: f64, t1912: f64, t25036: f64, t25188: f64, t25348: f64, t259: f64, t26591: f64, t28265: f64, t28269: f64, t28274: f64, t28278: f64, t4147: f64, t4268: f64, t7517: f64, t7538: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t28282, t28288, t28289, t28294, t28295, t28296, t28298, t28299, t28300) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1920(t1902, t5558, t25224, t7479, t6552, t23195, t5636, t6553, t1880, t5527, t6554, t23035);
        let t28304 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1921(t1528, t17052, t17092, t1912, t25036, t25188, t25348, t259, t26591, t28265, t28269, t28274, t28278, t28282, t28289, t28296, t28300, t4147, t4268, t7517, t7538);
    (t28282, t28288, t28294, t28295, t28298, t28299, t28304)
}
