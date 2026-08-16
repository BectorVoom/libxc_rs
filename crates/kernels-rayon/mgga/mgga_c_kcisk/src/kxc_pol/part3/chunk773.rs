//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 773/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk773(t1860: f64, t4597: f64, t3290: f64, t5248: f64, t10459: f64, t41: f64, t10463: f64, t702: f64, t10441: f64, t5172: f64, t695: f64, t1060: f64, t1919: f64) -> (f64, f64, f64) {
    let t11905 = t1860 * t4597;
    let t11907 = t5248 * t11905 * t3290;
    let t11910 = t41 * t10459;
    let t11911 = t702 * t10463;
    let t11913 = t11910 * t11911 * t10441;
    let t11916 = t5172 * t695;
    let t11918 = t1919 * t11916 * t1060;
    (t11907, t11913, t11918)
}
