//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1561;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1562;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta431(t22751: f64, t6892: f64, t6883: f64, t6908: f64, t22674: f64, t6891: f64, t22892: f64, t1988: f64, t22716: f64, t22724: f64, t6898: f64, t6902: f64, t794: f64, t6897: f64, t225: f64, t3886: f64, t6903: f64, t25: f64, t2752: f64, t1887: f64, t6581: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22908, t22910, t22920, t22922, t22923, t22925, t22927) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1561(t22751, t6892, t6883, t6908, t22674, t6891, t22892, t1988, t22716, t22724, t6898, t6902, t794);
        let (t22928, t22933, t22941, t22960) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1562(t22927, t6897, t225, t3886, t6883, t6903, t25, t2752);
        let t22986 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1563(t1887, t6581);
    (t22908, t22910, t22920, t22922, t22923, t22925, t22927, t22928, t22933, t22941, t22960, t22986)
}
