//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 835/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk835(t3: f64, t8496: f64, t1873: f64, t7010: f64, t3941: f64, t8319: f64, t1401: f64, t8326: f64, t577: f64, t131: f64, t8306: f64) -> (f64, f64, f64, f64) {
    let t8497 = t3 * t8496;
    let t8503 = t7010 * t1873;
    let t8506 = 27.0_f64 * t3941 * t8319;
    let t8508 = 0.135e2_f64 * t1401 * t8326;
    let t8509 = 0.45e1_f64 * t8496 * t577 + 27.0_f64 * t8503 + t8506 + t8508;
    let t8513 = t131 * t8306;
    (t8497, t8508, t8509, t8513)
}
