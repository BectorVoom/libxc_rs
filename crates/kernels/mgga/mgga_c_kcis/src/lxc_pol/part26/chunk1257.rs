//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1257/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1257<F: Float>(t1607: F, t613: F, t1598: F, t18256: F, t251: F, t18210: F, t28815: F, t7968: F, t27563: F, t28714: F, t1370: F, t7984: F) -> (F, F, F, F, F, F) {
    let t99002 = t613 * t1607;
    let t99013 = t18256 * t251 * t1598;
    let t99023 = t18210 * t28815;
    let t99024 = t7968 * t99023;
    let t99035 = F::new(0.23168402777777777778e-3) * t28714 * t27563;
    let t99046 = t1370 * t7984;
    (t99002, t99013, t99023, t99024, t99035, t99046)
}
