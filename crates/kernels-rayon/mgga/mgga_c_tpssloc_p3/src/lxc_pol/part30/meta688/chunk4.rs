//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2189/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2189(t2020: f64, t97804: f64, t15868: f64, t1983: f64, t7753: f64, t22574: f64, t74032: f64, t8643: f64, t24999: f64, t4073: f64, t5361: f64, t7681: f64, t96842: f64, t96844: f64, t96846: f64, t97777: f64, t97779: f64, t97783: f64, t97785: f64, t97788: f64, t97792: f64, t97794: f64, t97796: f64, t97798: f64, t97800: f64, t97802: f64) -> f64 {
    let t97805 = t97804 * t2020;
    let t97808 = 2.0_f64 * t1983 * t7753 * t15868;
    let t97811 = 3.0_f64 * t22574 * t8643 * t74032;
    let t97814 = -4.0_f64 * t24999 * t4073 + 2.0_f64 * t5361 * t7681 - t96842 - t96844 - t96846 + t97777 - t97779 - t97783 - t97785 - t97788 - t97792 + t97794 - t97796 - t97798 - t97800 - t97802 + t97805 - t97808 - t97811;
    t97814
}
