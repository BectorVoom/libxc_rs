//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 773/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk773(t2484: f64, t4663: f64, t1846: f64, t2477: f64, t2488: f64, t5082: f64, t2063: f64, t5101: f64, t2497: f64, t3119: f64, t2502: f64, t3123: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16037 = t4663 * t2484;
    let t16088 = t1846 * t2477;
    let t16090 = t5082 * t2488;
    let t16099 = t5101 * t2063;
    let t16204 = t3119 * t2497;
    let t16206 = t3123 * t2502;
    (t16037, t16088, t16090, t16099, t16204, t16206)
}
