//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 403/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk403(t1517: f64, t1518: f64, t833: f64, t1455: f64, t509: f64, t1153: f64, t1478: f64, t1483: f64, t1507: f64, t1516: f64, t368: f64, t545: f64, t562: f64, t86: f64) -> (f64, f64, f64) {
    let t1520 = t1517 * t1518 * t833;
    let t1523 = t509 * t1455;
    let t1527 = 0.619125e-2_f64 * t1507 * t545 + 0.9286875e-2_f64 * t562 * t1478 - 0.619125e-2_f64 * t562 * t1483 - t1516 - 0.26531111111111111111e-1_f64 * t1153 * t1520 - 0.39796666666666666666e-1_f64 * t86 * t368 * t1523;
    (t1520, t1523, t1527)
}
