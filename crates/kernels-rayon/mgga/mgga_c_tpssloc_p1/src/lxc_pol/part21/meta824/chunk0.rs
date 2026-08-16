//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2895/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2895(t2793: f64, t2842: f64, t5727: f64, t4395: f64, t2792: f64, t913: f64, t10650: f64, t14332: f64, t14436: f64, t14450: f64, t1581: f64, t2886: f64, t2888: f64, t4472: f64, t48776: f64, t48783: f64, t48854: f64, t49404: f64, t49478: f64, t60354: f64, t60359: f64, t60360: f64, t60371: f64, t60374: f64, t60377: f64, t60381: f64, t60384: f64, t60387: f64, t60391: f64, t931: f64) -> (f64, f64, f64, f64, f64) {
    let t60394 = 6.0_f64 * t2842 * t5727 * t2793;
    let t60395 = t4395 * t4395;
    let t60398 = 4.0_f64 * t2792 * t60395 * t913;
    let t60400 = 1.0_f64 * t10650 * t5727;
    let t60401 = 0.11696447245269292414e1_f64 * t49404 * t1581 + 0.23392894490538584828e1_f64 * t14332 * t4472 - t60354 - 0.77193501593724168323e3_f64 * t48776 * t14436 + t60359 + 0.64327917994770140268e2_f64 * t2886 * t60360 * t2888 + 0.14035736694323150897e2_f64 * t48783 * t14450 + 0.8276162067083744048e4_f64 * t49478 * t48854 * t931 + t60371 + t60374 - t60377 - t60381 - t60384 - t60387 - t60391 - t60394 + t60398 - t60400;
    (t60394, t60395, t60398, t60400, t60401)
}
