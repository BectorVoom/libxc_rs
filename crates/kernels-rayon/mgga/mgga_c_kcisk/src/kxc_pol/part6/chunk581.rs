//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 581/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk581(t1339: f64, t8086: f64, t1341: f64, t7744: f64, t1340: f64, t7736: f64, t3759: f64, t425: f64, t7706: f64, t2191: f64, t5646: f64, t7710: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8087 = t1339 * t8086;
    let t8089 = t1341 * t7744;
    let t8090 = t1340 * t8089;
    let t8091 = t1339 * t8090;
    let t8093 = t1341 * t7736;
    let t8094 = t1340 * t8093;
    let t8095 = t3759 * t8094;
    let t8099 = t425 * t7706;
    let t8102 = t5646 * t2191;
    let t8105 = t425 * t7710;
    (t8087, t8089, t8090, t8091, t8093, t8094, t8095, t8099, t8102, t8105)
}
