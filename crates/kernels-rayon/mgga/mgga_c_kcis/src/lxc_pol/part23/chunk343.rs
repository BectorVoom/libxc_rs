//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 343/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk343(t1517: f64, t1518: f64, t1650: f64, t1979: f64, t509: f64, t1153: f64, t1516: f64, t1991: f64, t1995: f64, t2018: f64, t368: f64, t545: f64, t562: f64, t86: f64) -> (f64, f64, f64) {
    let t2026 = t1517 * t1518 * t1650;
    let t2029 = t509 * t1979;
    let t2033 = 0.619125e-2_f64 * t2018 * t545 + 0.9286875e-2_f64 * t562 * t1991 - 0.619125e-2_f64 * t562 * t1995 - t1516 - 0.26531111111111111111e-1_f64 * t1153 * t2026 - 0.39796666666666666666e-1_f64 * t86 * t368 * t2029;
    (t2026, t2029, t2033)
}
