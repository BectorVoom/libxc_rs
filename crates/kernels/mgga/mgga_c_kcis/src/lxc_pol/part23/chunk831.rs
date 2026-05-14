//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 831/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk831<F: Float>(t16829: F, t2194: F, t1369: F, t3999: F, t1938: F, t3985: F, t498: F, t531: F, t737: F, t110: F, t1939: F, t493: F, t1930: F, t3974: F, t2469: F, t5714: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16830 = t16829 * t2194;
    let t16831 = t1369 * t3999;
    let t16832 = t16831 * t1938;
    let t16833 = t16832 * t3985;
    let t16836 = t1369 * t498;
    let t16837 = t16836 * t531;
    let t16838 = t737 * t16837;
    let t16841 = t110 * t1939;
    let t16842 = t493 * t16841;
    let t16845 = t1930 * t3974 / 54.0;
    let t16848 = t2469 * t1369;
    let t16849 = t16848 * t5714;
    (t16830, t16831, t16833, t16836, t16838, t16842, t16845, t16848, t16849)
}
