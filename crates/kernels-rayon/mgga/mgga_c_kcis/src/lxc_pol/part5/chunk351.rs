//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 351/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk351(t1307: f64, t1330: f64, t26: f64, t1309: f64, t1320: f64, t1322: f64, t1325: f64, t1329: f64) -> (f64, f64, f64) {
    let t1331 = t1330 * t1307;
    let t1332 = t26 * t1331;
    let t1334 = 0.1898925e1_f64 * t1320 - t1322 - 0.29896666666666666667e0_f64 * t1309 + 0.3071625e0_f64 * t1325 - t1329 - 0.82156666666666666667e-1_f64 * t1332;
    (t1331, t1332, t1334)
}
