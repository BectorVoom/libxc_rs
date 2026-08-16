//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1344/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1344(t22043: f64, t22089: f64, t22146: f64, t22210: f64, t1396: f64, t1468: f64, t1464: f64, t1489: f64, t21886: f64, t1494: f64, t7052: f64, t1497: f64) -> (f64, f64, f64, f64) {
    let t22212 = t22043 + t22089 + t22146 + t22210;
    let t22213 = t1396 * t22212;
    let t22214 = t1468 * t22213;
    let t22215 = t1464 * t22214;
    let t22219 = t21886 * t1489;
    let t22220 = t1468 * t22219;
    let t22221 = t1464 * t22220;
    let t22223 = t7052 * t1494;
    let t22224 = t22223 * t1497;
    (t22212, t22215, t22221, t22224)
}
