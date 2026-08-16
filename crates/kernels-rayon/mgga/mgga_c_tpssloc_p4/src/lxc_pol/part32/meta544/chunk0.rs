//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1892/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1892(t27710: f64, t7325: f64, t2132: f64, t25588: f64, t2121: f64, t2136: f64, t24650: f64, t24747: f64, t24752: f64, t24754: f64, t27681: f64, t27684: f64, t27687: f64, t27692: f64, t27697: f64, t27701: f64, t27704: f64, t4989: f64, t7321: f64, t7326: f64, t7331: f64, t7345: f64, t8040: f64) -> (f64, f64, f64) {
    let t27711 = t27710 * t7325;
    let t27714 = t2132 * t25588;
    let t27719 = -0.80745512188280781712e-3_f64 * t27681 - 0.10093189023535097714e-3_f64 * t27684 * t7331 - t2121 * t27687 / 144.0_f64 + 0.10093189023535097714e-3_f64 * t7326 * t27692 - 0.10093189023535097714e-3_f64 * t24650 * t8040 + t2121 * t27697 / 216.0_f64 + 0.10093189023535097714e-3_f64 * t27701 - 0.10093189023535097714e-3_f64 * t27704 * t7321 + 5.0_f64 / 6912.0_f64 * t7345 * t4989 - 0.10093189023535097714e-3_f64 * t24747 - 0.80745512188280781712e-3_f64 * t27711 * t7331 - 0.10093189023535097714e-3_f64 * t27714 * t2136 - t24752 / 3456.0_f64 + t24754 / 2304.0_f64;
    (t27711, t27714, t27719)
}
