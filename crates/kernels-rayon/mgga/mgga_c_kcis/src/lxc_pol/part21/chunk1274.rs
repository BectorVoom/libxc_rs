//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1274/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1274(t3330: f64, t5189: f64, t7766: f64, t27999: f64, t33853: f64, t14665: f64, t1820: f64, t93243: f64, t10498: f64, t1203: f64, t28005: f64, t14683: f64, t26871: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95498 = 4.0_f64 * t3330 * t7766 * t5189;
    let t95500 = 12.0_f64 * t33853 * t27999;
    let t95502 = 2.0_f64 * t14665 * t7766;
    let t95503 = t93243 * t1820;
    let t95506 = 12.0_f64 * t10498 * t28005 * t1203;
    let t95508 = 2.0_f64 * t26871 * t14683;
    (t95498, t95500, t95502, t95503, t95506, t95508)
}
