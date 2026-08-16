//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 356/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk356(t1305: f64, t1328: f64, t1309: f64, t1320: f64, t1325: f64, t1332: f64) -> (f64, f64, f64) {
    let t1349 = 0.301925e0_f64 * t1305;
    let t1352 = 0.82785e-1_f64 * t1328;
    let t1354 = 0.258925e1_f64 * t1320 - t1349 - 0.301925e0_f64 * t1309 + 0.16504875e0_f64 * t1325 - t1352 - 0.82785e-1_f64 * t1332;
    (t1349, t1352, t1354)
}
