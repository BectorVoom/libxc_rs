//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1070/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1070(t18210: f64, t7985: f64, t7978: f64, t27563: f64, t251: f64, t4409: f64, t1598: f64) -> (f64, f64, f64, f64, f64) {
    let t27601 = t18210 * t7985;
    let t27602 = t7978 * t27601;
    let t27604 = t7978 * t27563;
    let t27606 = t4409 * t251;
    let t27607 = t27606 * t1598;
    (t27601, t27602, t27604, t27606, t27607)
}
