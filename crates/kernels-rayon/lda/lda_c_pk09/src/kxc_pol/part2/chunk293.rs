//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 293/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk293(t1263: f64, t1272: f64, t1268: f64, t1275: f64, t299: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1295 = 11.879313099038017_f64 * t1263;
    let t1297 = 3.959771033012672_f64 * t1272;
    let t1299 = t1295 - 11.879313099038017_f64 * t1268 + t1297 + 11.879313099038017_f64 * t1275;
    let t1300 = t299 * t299;
    let t1301 = t1300 + 1.0_f64;
    let t1302 = 1.0_f64 / t1301;
    let t1303 = t1299 * t1302;
    (t1295, t1297, t1299, t1301, t1302, t1303)
}
