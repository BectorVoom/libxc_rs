//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 619/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk619(t1322: f64, t5081: f64, t1477: f64, t747: f64, t1513: f64, t1216: f64, t1223: f64, t1311: f64, t4979: f64, t1314: f64, t5031: f64, t1287: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5082 = t1322 * t5081;
    let t5084 = t747 * t1477;
    let t5085 = t1513 * t5084;
    let t5087 = t1216 * t1223;
    let t5090 = 1.8805371096875316_f64 * t1311 * t4979;
    let t5091 = t1314 * t5031;
    let t5092 = t5091 * t1287;
    (t5082, t5084, t5085, t5087, t5090, t5092)
}
