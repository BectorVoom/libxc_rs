//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta747 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2618;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta747(t3566: f64, t5023: f64, t15734: f64, t3490: f64, t11789: f64, t1227: f64, t248: f64, t4733: f64, t11712: f64, t11913: f64, t491: f64, t11887: f64, t52834: f64, t11880: f64, t15831: f64, t225: f64, t11605: f64, t1760: f64, t15816: f64, t15908: f64, t9467: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53507, t53515, t53519, t53545, t53565) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2618(t3566, t5023, t15734, t3490, t11789, t1227, t248, t4733, t11712, t11913, t491, t11887, t52834);
        let (t53592, t53613, t53646, t53658, t53677, t53703, t53777) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2619(t11913, t52834, t11880, t11712, t11887, t491, t15831, t225, t11605, t1760, t15816, t15908, t9467);
    (t53507, t53515, t53519, t53545, t53565, t53592, t53613, t53646, t53658, t53677, t53703, t53777)
}
