//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1082/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1082<F: Float>(t18210: F, t7985: F, t7978: F, t27563: F, t251: F, t4409: F, t1598: F) -> (F, F, F, F, F) {
    let t27601 = t18210 * t7985;
    let t27602 = t7978 * t27601;
    let t27604 = t7978 * t27563;
    let t27606 = t4409 * t251;
    let t27607 = t27606 * t1598;
    (t27601, t27602, t27604, t27606, t27607)
}
