//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1623;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1624;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1625;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta298(t2775: f64, t283: f64, t61: f64, t135: f64, t3142: f64, t973: f64, t3147: f64, t3152: f64, t248: f64, t3101: f64, t3132: f64, t3130: f64, t225: f64, t3167: f64, t10947: f64, t3185: f64, t3199: f64, t1014: f64, t10471: f64, t10470: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10969, t10970, t10982, t10985, t10994, t11002, t11003) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1623(t2775, t283, t61, t135, t3142, t973, t3147, t3152, t248, t3101, t3132, t3130);
        let (t11010, t11034) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1624(t225, t3167, t10947, t3185);
        let t11037 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1625(t10947, t3199);
        let (t11045, t11046) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1626(t1014, t10471, t10470);
    (t10969, t10970, t10982, t10985, t10994, t11002, t11003, t11010, t11034, t11037, t11045, t11046)
}
