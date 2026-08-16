//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1814;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta578(t26339: f64, t81159: f64, t22716: f64, t7697: f64, t26216: f64, t26210: f64, t6897: f64, t794: f64, t1377: f64, t5187: f64, t7692: f64, t81186: f64, t26338: f64, t81228: f64, t81326: f64, t22892: f64, t7691: f64, t80645: f64, t26206: f64, t6883: f64, t1834: f64, t6891: f64, t22704: f64, t26355: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90500, t90503, t90511, t90514, t90516, t90521) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1814(t26339, t81159, t22716, t7697, t26216, t26210, t6897, t794, t1377, t5187, t7692, t81186);
        let (t90524, t90533, t90541, t90544, t90546, t90549) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1815(t26338, t81228, t81326, t22892, t7691, t80645, t26206, t6883, t1834, t794, t6891, t22704, t26355);
    (t90500, t90503, t90511, t90514, t90516, t90521, t90524, t90533, t90541, t90544, t90546, t90549)
}
