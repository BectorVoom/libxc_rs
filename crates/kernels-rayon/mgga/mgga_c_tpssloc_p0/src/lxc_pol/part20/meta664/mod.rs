//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta664 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2485;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2486;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2487;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2488;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2489;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2490;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2491;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta664(t14730: f64, t9288: f64, t1113: f64, t136: f64, t12606: f64, t3242: f64, t607: f64, t3297: f64, t123: f64, t3240: f64, t50857: f64, t50861: f64, t43835: f64, t43837: f64, t43839: f64, t43855: f64, t43857: f64, t43859: f64, t43861: f64, t43863: f64, t14750: f64, t690: f64, t14754: f64, t14745: f64, t11147: f64, t2244: f64, t3966: f64, t11145: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50879, t50881, t50884, t50886, t50897) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2485(t14730, t9288, t1113, t136, t12606, t3242, t607, t3297, t123, t3240, t50857);
        let t50900 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2486(t123, t3240, t50861);
        let t50902 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2487(t43835, t43837, t43839, t43855, t43857, t43859, t43861, t43863, t50881, t50886, t50897, t50900);
        let t50903 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2488(t14750, t690);
        let t50905 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2489(t14754, t690);
        let t50907 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2490(t14745, t690);
        let (t50910, t50912) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2491(t11147, t2244, t3966, t11145, t123);
    (t50879, t50881, t50884, t50886, t50897, t50900, t50902, t50903, t50905, t50907, t50910, t50912)
}
