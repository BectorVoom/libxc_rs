//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 638/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk638(t1287: f64, t5391: f64, t1487: f64, t4979: f64, t1481: f64, t4982: f64, t332: f64, t5081: f64, t1525: f64, t5294: f64, t1435: f64, t1543: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5392 = t5391 * t1287;
    let t5395 = 2.427516195194328_f64 * t1487 * t4979;
    let t5396 = t1481 * t4982;
    let t5404 = t332 * t5081;
    let t5408 = t1525 * t5294;
    let t5409 = 5.40024514194619_f64 * t5408;
    let t5414 = t1543 * t1435;
    (t5392, t5395, t5396, t5404, t5408, t5409, t5414)
}
