//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta72 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk434;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk435;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk436;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk437;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta72(t480: f64, t11: f64, t2: f64, t584: f64, t16: f64, t9: f64, t587: f64, t591: f64, t14: f64, t21: f64, t594: f64, t598: f64, t15: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2130, t2218, t2219, t2220, t2221) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk434(t480, t11, t2, t584, t16, t9);
        let (t2222, t2223) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk435(t2221, t587, t591);
        let (t2224, t2225) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk436(t2223, t14, t21);
        let (t2226, t2228, t2229) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk437(t2225, t594, t598, t15);
    (t2130, t2218, t2219, t2220, t2221, t2222, t2223, t2224, t2225, t2226, t2228, t2229)
}
