//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1194/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1194(t27556: f64, t27563: f64, t27607: f64, t27566: f64, t27606: f64, t27555: f64, t18171: f64, t27568: f64, t27567: f64, t27594: f64, t54162: f64, t7978: f64, t7985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t94916 = t27556 * t27563;
    let t94919 = t27607 * t27563;
    let t94928 = t27606 * t27566;
    let t94931 = t27555 * t27566;
    let t94934 = t18171 * t27568;
    let t94935 = t27567 * t94934;
    let t94966 = t27594 * t27566;
    let t94974 = t7978 * t54162 * t7985;
    (t94916, t94919, t94928, t94931, t94934, t94935, t94966, t94974)
}
