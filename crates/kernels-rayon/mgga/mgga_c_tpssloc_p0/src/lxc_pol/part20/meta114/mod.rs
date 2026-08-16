//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta114 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk763;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk764;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta114(t2807: f64, t894: f64, t2764: f64, t273: f64, t2799: f64, t901: f64, t241: f64, t63: f64, t281: f64, t283: f64, t699: f64, t909: f64, t976: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2808, t2810, t2815) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk763(t2807, t894, t2764, t273);
        let (t2816, t2818, t2820, t2822, t2823, t2824) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk764(t2799, t2815, t2807, t901, t241, t63, t281, t283, t699, t909);
        let t2826 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk765(t241, t976);
    (t2808, t2810, t2815, t2816, t2818, t2820, t2822, t2823, t2824, t2826)
}
