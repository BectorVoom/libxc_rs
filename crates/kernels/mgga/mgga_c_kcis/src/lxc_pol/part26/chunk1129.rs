//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1129/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1129<F: Float>(t27563: F, t28853: F, t1598: F, t251: F, t54624: F, t1607: F, t613: F, t18256: F, t18210: F, t28815: F, t7968: F, t28714: F, t1370: F, t7984: F, t98057: F, t28737: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t98988 = 0.82448622685185185186e-4 * t28853 * t27563;
    let t98994 = t54624 * t251 * t1598;
    let t99002 = t613 * t1607;
    let t99013 = t18256 * t251 * t1598;
    let t99023 = t18210 * t28815;
    let t99024 = t7968 * t99023;
    let t99035 = 0.23168402777777777778e-3 * t28714 * t27563;
    let t99046 = t1370 * t7984;
    let t99052 = 0.15476481481481481481e-2 * t98057;
    let t99056 = t18210 * t28737;
    (t98988, t98994, t99002, t99013, t99023, t99024, t99035, t99046, t99052, t99056)
}
