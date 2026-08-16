//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 839/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk839<F: Float>(t11041: F, t1251: F, t110: F, t992: F, t1254: F, t3500: F, t3525: F, t25: F, t2887: F, t3509: F, t3530: F, t993: F) -> (F, F, F, F, F, F, F) {
    let t11042 = t1251 * t11041;
    let t11061 = t110 * t992;
    let t11062 = t11061 * t1254;
    let t11063 = t1251 * t11062;
    let t11065 = t3500 * t3525;
    let t11066 = t1251 * t11065;
    let t11068 = t25 * t2887;
    let t11069 = t11068 * t3509;
    let t11070 = t1251 * t11069;
    let t11072 = t993 * t3530;
    (t11042, t11061, t11063, t11066, t11068, t11070, t11072)
}
