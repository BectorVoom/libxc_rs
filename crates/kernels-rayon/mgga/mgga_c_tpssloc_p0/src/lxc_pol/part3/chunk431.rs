//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 431/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk431(t1352: f64, t1380: f64, t1372: f64, t553: f64, t1332: f64, t1336: f64, t544: f64, t564: f64, t1378: f64) -> (f64, f64, f64, f64) {
    let t1381 = t1380 * t1352;
    let t1383 = t553 * t1372;
    let t1385 = t1332 * t564 - t1336 * t1381 + t1383 * t544;
    let t1386 = t1378 * t1385;
    (t1381, t1383, t1385, t1386)
}
