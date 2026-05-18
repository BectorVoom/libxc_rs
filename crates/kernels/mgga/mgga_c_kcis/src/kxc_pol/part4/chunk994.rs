//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 994/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk994<F: Float>(t11062: F, t1251: F, t3500: F, t3525: F, t25: F, t2887: F, t3509: F, t3530: F, t993: F, t1259: F, t2880: F, t3516: F) -> (F, F, F, F, F, F) {
    let t11063 = t1251 * t11062;
    let t11065 = t3500 * t3525;
    let t11066 = t1251 * t11065;
    let t11068 = t25 * t2887;
    let t11069 = t11068 * t3509;
    let t11070 = t1251 * t11069;
    let t11072 = t993 * t3530;
    let t11081 = t2880 * t1259;
    let t11082 = t11081 * t3516;
    (t11063, t11066, t11070, t11072, t11081, t11082)
}
