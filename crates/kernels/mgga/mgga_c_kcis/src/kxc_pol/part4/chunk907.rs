//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 907/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk907<F: Float>(t3490: F, t3501: F, t3500: F, t3521: F, t1251: F, t110: F, t992: F, t1254: F, t3525: F, t25: F, t2887: F, t3509: F, t3530: F, t993: F, t1259: F, t2880: F) -> (F, F, F, F, F, F, F, F) {
    let t11034 = t3490 * t3501;
    let t11041 = t3500 * t3521;
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
    let t11081 = t2880 * t1259;
    (t11034, t11042, t11061, t11063, t11066, t11070, t11072, t11081)
}
