//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta97 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk598;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk599;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk600;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk601;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk602;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta97(t2219: f64, t16: f64, t9: f64, t587: f64, t591: f64, t14: f64, t21: f64, t594: f64, t598: f64, t15: f64, t19: f64, t601: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2220, t2221) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk598(t2219, t16, t9);
        let (t2222, t2223, t2224, t2225) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk599(t2221, t587, t591, t14, t21);
        let (t2226, t2228, t2229) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk600(t2225, t594, t598, t15);
        let t2230 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk601(t2229);
        let (t2232, t2235) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk602(t19, t2230, t601, t604);
    (t2220, t2221, t2222, t2223, t2224, t2225, t2226, t2228, t2229, t2230, t2232, t2235)
}
