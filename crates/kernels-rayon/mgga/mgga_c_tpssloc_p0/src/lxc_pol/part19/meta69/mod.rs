//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta69 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk428;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk429;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk430;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk431;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk432;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk433;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk434;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta69(t11: f64, t2: f64, t584: f64, t16: f64, t9: f64, t587: f64, t591: f64, t14: f64, t21: f64, t594: f64, t598: f64, t15: f64, t19: f64, t601: f64, t604: f64, t84: f64, t85: f64, t24: f64, t645: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2218, t2219, t2220, t2221) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk428(t11, t2, t584, t16, t9);
        let (t2222, t2223) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk429(t2221, t587, t591);
        let (t2224, t2225) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk430(t2223, t14, t21);
        let (t2226, t2228, t2229) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk431(t2225, t594, t598, t15);
        let (t2230, t2233, t2235, t2239) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk432(t2229, t19, t2218, t2220, t2222, t2224, t2226, t2228, t601, t604, t84, t85);
        let (t2240, t2241) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk433(t2239, t24, t645);
        let t2244 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk434(t607);
    (t2219, t2221, t2223, t2225, t2229, t2230, t2233, t2235, t2239, t2240, t2241, t2244)
}
