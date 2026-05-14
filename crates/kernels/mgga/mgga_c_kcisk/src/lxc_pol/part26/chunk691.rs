//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 691/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk691<F: Float>(t1450: F, t8077: F, t1415: F, t1411: F, t3776: F, t8010: F, t1340: F, t2177: F, t5606: F, t1339: F, t1341: F, t7744: F, t7736: F, t3759: F, t425: F, t7706: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8078 = t1450 * t8077;
    let t8079 = t1415 * t8078;
    let t8080 = t1411 * t8079;
    let t8082 = t3776 * t8010;
    let t8083 = t1340 * t8082;
    let t8084 = t1411 * t8083;
    let t8086 = t5606 * t2177;
    let t8087 = t1339 * t8086;
    let t8089 = t1341 * t7744;
    let t8090 = t1340 * t8089;
    let t8091 = t1339 * t8090;
    let t8093 = t1341 * t7736;
    let t8094 = t1340 * t8093;
    let t8095 = t3759 * t8094;
    let t8099 = t425 * t7706;
    (t8078, t8079, t8080, t8082, t8083, t8084, t8086, t8087, t8089, t8090, t8091, t8093, t8094, t8095, t8099)
}
