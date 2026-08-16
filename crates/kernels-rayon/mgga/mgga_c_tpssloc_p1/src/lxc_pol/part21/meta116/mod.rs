//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta116 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk794;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk795;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk796;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk797;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk798;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta116(t290: f64, t2793: f64, t2842: f64, t2764: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64, t919: f64, t923: f64, t307: f64, t922: f64, t302: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2843, t2844) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk794(t290);
        let (t2845, t2847, t2848, t2853, t2856) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk795(t2793, t2844, t2842, t2764, t2766, t2773, t2778, t2782, t919, t923);
        let (t2859, t2860) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk796(t307, t922);
        let t2861 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk797(t2860, t302);
        let t2862 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk798(t931);
    (t2843, t2844, t2845, t2847, t2848, t2853, t2856, t2859, t2860, t2861, t2862)
}
