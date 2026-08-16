//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta741 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2606;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta741(t11791: f64, t5024: f64, t11820: f64, t5002: f64, t11153: f64, t4899: f64, t3540: f64, t4961: f64, t11709: f64, t15640: f64, t1227: f64, t13969: f64, t15611: f64, t15454: f64, t4973: f64, t49850: f64, t11678: f64, t11697: f64, t15559: f64, t15713: f64, t3577: f64, t45124: f64, t1213: f64, t1735: f64, t248: f64, t45017: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52991, t52993, t52995, t52999, t53001, t53023) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2606(t11791, t5024, t11820, t5002, t11153, t4899, t3540, t4961, t11709, t15640, t1227, t13969, t15611);
        let (t53026, t53033, t53064, t53067, t53079) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2607(t1227, t13969, t15454, t4973, t49850, t11678, t11697, t15559, t15713, t3577, t45124, t1213, t1735, t248, t45017);
    (t52991, t52993, t52995, t52999, t53001, t53023, t53026, t53033, t53064, t53067, t53079)
}
