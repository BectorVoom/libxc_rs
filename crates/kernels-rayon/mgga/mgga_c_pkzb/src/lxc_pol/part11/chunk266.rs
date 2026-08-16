//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 266/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk266(t237: f64, t365: f64, t830: f64, t855: f64, t858: f64, t863: f64, t872: f64, t878: f64, t882: f64, t891: f64, t369: f64) -> (f64, f64, f64) {
    let t895 = t237 * (-0.310907e-1_f64 * t858 * t365 + 1.0_f64 * t863 * t872 + t830 - t855 - 0.19751673498613801407e-1_f64 * t878 + 0.5848223622634646207e0_f64 * t882 * t891);
    let t897 = 0.19751673498613801407e-1_f64 * t237 * t878;
    let t898 = t237 * t369;
    (t895, t897, t898)
}
