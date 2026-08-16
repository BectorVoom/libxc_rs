//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1369/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1369(t1517: f64, t1650: f64, t17546: f64, t12371: f64, t6281: f64, t4225: f64, t6284: f64, t1518: f64, t18431: f64, t21786: f64, t509: f64, t2018: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22547 = t1517 * t17546 * t1650;
    let t22554 = t1517 * t12371 * t6281;
    let t22558 = t1517 * t4225 * t6284;
    let t22562 = t1517 * t1518 * t18431;
    let t22570 = t509 * t21786;
    let t22574 = t2018 * t543;
    (t22547, t22554, t22558, t22562, t22570, t22574)
}
