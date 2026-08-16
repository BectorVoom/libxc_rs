//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1947;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta661(t23168: f64, t28277: f64, t28295: f64, t6547: f64, t6562: f64, t7488: f64, t86893: f64, t28439: f64, t28268: f64, t81591: f64, t17049: f64, t1880: f64, t6553: f64, t6571: f64, t1527: f64, t776: f64, t23270: f64, t25038: f64, t25191: f64, t23204: f64, t28294: f64, t1493: f64, t254: f64, t28263: f64, t23237: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98921, t98923, t98927, t98932, t98941, t98945) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1947(t23168, t28277, t28295, t6547, t6562, t7488, t86893, t28439, t28268, t81591, t17049, t1880, t6553, t6571);
        let (t98963, t98966, t98975, t98983, t98986) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1948(t1527, t776, t23270, t25038, t25191, t23204, t28294, t6562, t1493, t254, t28263, t1880, t23237);
    (t98921, t98923, t98927, t98932, t98941, t98945, t98963, t98966, t98975, t98983, t98986)
}
