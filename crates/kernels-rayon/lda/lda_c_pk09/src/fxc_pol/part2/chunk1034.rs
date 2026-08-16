//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1034/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1034(t11191: f64, t93: f64, t1836: f64, t1729: f64, t2747: f64, t11165: f64, t11168: f64, t11172: f64, t11177: f64, t11179: f64, t11184: f64, t11187: f64, t1783: f64, t1842: f64, t2032: f64, t6275: f64, t6280: f64, t6294: f64, t6304: f64, t6308: f64, t6478: f64, t6480: f64, t6483: f64, t6487: f64, t6490: f64) -> f64 {
    let t11192 = t93 * t11191;
    let t11193 = t1836 * t11192;
    let t11195 = t2747 * t1729;
    let t11196 = t93 * t11195;
    let t11201 = 14.216351496367702_f64 * t6275 - 14.216351496367702_f64 * t6280 + 14.216351496367702_f64 * t11165 + 3.5540878740919255_f64 * t1783 * t93 * t11168 + 2.9824072957409817_f64 * t11172 * t2032 - 0.15277772349540736_f64 * t11177 * t11179 + 14.216351496367702_f64 * t1842 * t11184 + 3.5540878740919255_f64 * t1783 * t93 * t11187 - 14.216351496367702_f64 * t11193 + 14.216351496367702_f64 * t1842 * t11196 - t6294 + t6304 - t6308 + 1.6457779058161184_f64 * t6478 - 1.6457779058161184_f64 * t6480 - t6483 + t6487 + t6490;
    t11201
}
