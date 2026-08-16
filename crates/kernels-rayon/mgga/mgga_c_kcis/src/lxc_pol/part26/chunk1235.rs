//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1235/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1235(t27555: f64, t27566: f64, t3754: f64, t4314: f64, t27594: f64, t54162: f64, t7978: f64, t7985: f64, t7970: f64, t7968: f64, t12825: f64, t7980: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94931 = t27555 * t27566;
    let t94960 = t4314 * t3754;
    let t94966 = t27594 * t27566;
    let t94974 = t7978 * t54162 * t7985;
    let t94976 = t54162 * t7970;
    let t94977 = t7978 * t94976;
    let t94979 = t7968 * t94976;
    let t95001 = t7978 * t12825 * t7980;
    (t94931, t94960, t94966, t94974, t94977, t94979, t95001)
}
