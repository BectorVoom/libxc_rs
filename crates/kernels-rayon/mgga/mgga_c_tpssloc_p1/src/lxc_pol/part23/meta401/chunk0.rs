//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1210/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1210(t10189: f64, t5842: f64, t5836: f64, t5838: f64, t698: f64, t973: f64, t5844: f64, t4509: f64, t10224: f64, t5824: f64, t2986: f64, t4514: f64, t48019: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t61189 = t10189 * t5842;
    let t61250 = t10189 * t5836;
    let t61310 = t973 * t698 * t5838;
    let t61313 = t973 * t698 * t5844;
    let t61322 = t4509 * t5836;
    let t61365 = t4509 * t5842;
    let t61408 = t973 * t10224 * t5824;
    let t61489 = t2986 * t48019 * t4514;
    (t61189, t61250, t61310, t61313, t61322, t61365, t61408, t61489)
}
