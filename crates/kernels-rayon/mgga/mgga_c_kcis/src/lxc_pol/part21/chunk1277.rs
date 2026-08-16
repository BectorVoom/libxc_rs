//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1277/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1277(t283: f64, t33822: f64, t990: f64, t27806: f64, t2809: f64, t42625: f64, t1092: f64, t19575: f64, t26760: f64, t3219: f64, t14650: f64, t1008: f64, t829: f64) -> (f64, f64, f64, f64, f64) {
    let t95535 = t33822 * t283 * t990;
    let t95537 = t27806 * t42625 * t2809;
    let t95542 = t1092 * t26760 * t19575 * t3219;
    let t95545 = t1092 * t26760 * t14650;
    let t95547 = t829 * t1008;
    (t95535, t95537, t95542, t95545, t95547)
}
