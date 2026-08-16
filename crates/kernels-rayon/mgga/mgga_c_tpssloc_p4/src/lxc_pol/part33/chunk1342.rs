//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1342/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1342(t1599: f64, t1921: f64, t1956: f64, t21130: f64, t21677: f64, t23327: f64, t25784: f64, t28701: f64, t387: f64, t5836: f64, t5844: f64, t6687: f64, t6690: f64, t6771: f64, t69871: f64, t70980: f64, t82391: f64, t82436: f64, t88162: f64, t88731: f64, t99205: f64, t99230: f64, t99297: f64) -> f64 {
    let t105890 = -6.0_f64 * t6771 * t21677 - t69871 * t1956 + 0.16449340668482264365e-1_f64 * t23327 * t88162 * t28701 - 0.16449340668482264365e-1_f64 * t99205 + 0.8529287754027840782e-2_f64 * t6687 * t82391 * t6690 * t21130 + t82436 - 0.49348022005446793095e-1_f64 * t6687 * t1599 * t99297 - 0.24674011002723396548e-1_f64 * t6687 * t1599 * t1921 * t387 * t5836 + 0.16449340668482264365e-1_f64 * t99230 - 3.0_f64 * t70980 * t1956 - 0.18277045187202515961e-2_f64 * t88731 + 0.24674011002723396548e-1_f64 * t6687 * t5844 * t25784;
    t105890
}
