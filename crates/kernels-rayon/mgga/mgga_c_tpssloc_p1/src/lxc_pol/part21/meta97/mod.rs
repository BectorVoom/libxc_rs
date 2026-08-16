//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta97 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk681;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk682;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk683;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk684;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta97(t138: f64, t681: f64, t125: f64, t2412: f64, t702: f64, t118: f64, t142: f64, t2393: f64, t706: f64, t717: f64, t708: f64, t607: f64, t751: f64, t707: f64, t195: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2418, t2419, t2420, t2421, t2423) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk681(t138, t681, t125, t2412, t702);
        let t2426 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk682(t118, t142, t2393);
        let t2427 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk683(t706, t717);
        let (t2429, t2430) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk684(t2427, t708, t607, t751);
        let (t2431, t2432, t2433) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk685(t2430, t707, t195);
    (t2418, t2419, t2420, t2421, t2423, t2426, t2427, t2429, t2430, t2431, t2432, t2433)
}
