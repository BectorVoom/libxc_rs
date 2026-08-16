//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 580/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk580(t1341: f64, t8072: f64, t3785: f64, t1411: f64, t2152: f64, t2231: f64, t1450: f64, t1415: f64, t3776: f64, t8010: f64, t1340: f64, t2177: f64, t5606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8073 = t1341 * t8072;
    let t8074 = t3785 * t8073;
    let t8075 = t1411 * t8074;
    let t8077 = t2231 * t2152;
    let t8078 = t1450 * t8077;
    let t8079 = t1415 * t8078;
    let t8080 = t1411 * t8079;
    let t8082 = t3776 * t8010;
    let t8083 = t1340 * t8082;
    let t8084 = t1411 * t8083;
    let t8086 = t5606 * t2177;
    (t8073, t8074, t8075, t8077, t8078, t8079, t8080, t8082, t8083, t8084, t8086)
}
