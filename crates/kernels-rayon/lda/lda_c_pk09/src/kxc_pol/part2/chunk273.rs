//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 273/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk273(t1222: f64, t94: f64, t332: f64, t225: f64, t282: f64, t68: f64, t10: f64, t9: f64, t215: f64, t599: f64, t221: f64, t584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1223 = t1222 * t94;
    let t1224 = t332 * t1223;
    let t1225 = 7.35994946043302_f64 * t1224;
    let t1226 = t225 * t282;
    let t1227 = t1226 * t68;
    let t1228 = t9 * t10;
    let t1232 = t215 * t599;
    let t1235 = t584 * t221;
    (t1223, t1224, t1225, t1226, t1227, t1228, t1232, t1235)
}
