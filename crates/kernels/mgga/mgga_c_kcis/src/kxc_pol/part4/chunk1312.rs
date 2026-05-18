//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1312/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1312<F: Float>(t16823: F, t3715: F, t20: F, t492: F, t2194: F, t1369: F, t3999: F, t1938: F, t3985: F, t498: F, t531: F, t737: F) -> (F, F, F, F) {
    let t16824 = t16823 * t3715;
    let t16829 = t492 * t20;
    let t16830 = t16829 * t2194;
    let t16831 = t1369 * t3999;
    let t16832 = t16831 * t1938;
    let t16833 = t16832 * t3985;
    let t16836 = t1369 * t498;
    let t16837 = t16836 * t531;
    let t16838 = t737 * t16837;
    (t16824, t16830, t16833, t16838)
}
