//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1312/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1312(t16823: f64, t3715: f64, t20: f64, t492: f64, t2194: f64, t1369: f64, t3999: f64, t1938: f64, t3985: f64, t498: f64, t531: f64, t737: f64) -> (f64, f64, f64, f64) {
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
