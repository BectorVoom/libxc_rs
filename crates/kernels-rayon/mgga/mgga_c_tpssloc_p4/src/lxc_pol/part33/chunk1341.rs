//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1341/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1341(t105776: f64, t105829: f64, t1634: f64, t5392: f64, t1052: f64, t1599: f64, t23327: f64, t23329: f64, t25442: f64, t28474: f64, t28515: f64, t28678: f64, t28697: f64, t28701: f64, t3174: f64, t4660: f64, t6687: f64, t7553: f64, t82342: f64, t88050: f64, t99131: f64, t99151: f64, t99184: f64, t99190: f64, t99273: f64, t99336: f64) -> (f64, f64, f64) {
    let t105830 = t105776 + t105829;
    let t105840 = t5392 * t1634;
    let t105863 = -0.82246703342411321826e-2_f64 * t23327 * t99336 * t7553 - 0.16449340668482264365e-1_f64 * t23327 * t88050 * t28701 - 0.82246703342411321826e-2_f64 * t23327 * t99273 * t7553 + 0.16449340668482264365e-1_f64 * t23327 * t23329 * t82342 * t105840 + 6.0_f64 * t1052 * t3174 * t28678 * t1634 - 0.82246703342411321826e-2_f64 * t99151 - 0.24674011002723396548e-1_f64 * t6687 * t1599 * t28474 - 0.82246703342411321826e-2_f64 * t23327 * t25442 * t28515 - 0.82246703342411321826e-2_f64 * t99184 - 0.54831135561607547883e-2_f64 * t99190 - 0.16449340668482264365e-1_f64 * t23327 * t99131 * t7553 - 18.0_f64 * t4660 * t28697;
    (t105830, t105840, t105863)
}
