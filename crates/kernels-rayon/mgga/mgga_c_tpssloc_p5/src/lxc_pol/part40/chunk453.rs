//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 453/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk453(t1519: f64, t218: f64, t1510: f64, t860: f64, t235: f64, t1499: f64, t226: f64, t255: f64, t812: f64, t858: f64, t1493: f64, t259: f64, t855: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1520 = t218 * t1519;
    let t1523 = t860 * t1510;
    let t1525 = t235 * t1519;
    let t1527 = t1499 * t255 - t1523 * t812 + t1525 * t226;
    let t1528 = t858 * t1527;
    let t1530 = t1493 * t259 + t1520 * t259 - t1528 * t855;
    (t1520, t1523, t1525, t1527, t1528, t1530)
}
