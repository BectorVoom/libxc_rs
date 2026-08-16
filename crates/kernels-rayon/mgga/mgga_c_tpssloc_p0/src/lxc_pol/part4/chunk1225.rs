//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1225/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1225(t20118: f64, t20147: f64, t3: f64, t1851: f64, t1858: f64, t576: f64, t6483: f64, t112: f64, t6470: f64, t671: f64, t1458: f64, t4072: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20148 = t20118 + t20147;
    let t20149 = t3 * t20148;
    let t20152 = t1851 * t1858;
    let t20158 = t576 * t6483;
    let t20162 = t6470 * t112;
    let t20173 = t576 * t671;
    let t20176 = t1458 * t4072;
    (t20148, t20149, t20152, t20158, t20162, t20173, t20176)
}
