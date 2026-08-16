//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 567/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk567(t291: f64, t4351: f64, t1543: f64, t892: f64, t914: f64, t1557: f64, t2787: f64, t912: f64, t2792: f64, t1547: f64, t2798: f64, t896: f64) -> (f64, f64, f64, f64, f64) {
    let t4353 = 0.621814e-1_f64 * t4351 * t291;
    let t4354 = t1543 * t892;
    let t4356 = 1.0_f64 * t4354 * t914;
    let t4358 = 1.0_f64 * t2787 * t1557;
    let t4359 = t1557 * t912;
    let t4361 = 2.0_f64 * t2792 * t4359;
    let t4362 = t2798 * t1547;
    let t4363 = t4362 * t896;
    (t4353, t4356, t4358, t4361, t4363)
}
