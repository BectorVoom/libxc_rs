//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 987/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk987(t10576: f64, t306: f64, t1322: f64, t9836: f64, t311: f64, t5035: f64, t5082: f64, t5085: f64, t5087: f64, t5090: f64, t5092: f64, t5095: f64, t5104: f64, t5106: f64, t5115: f64, t5119: f64, t5121: f64, t5124: f64, t5225: f64, t5235: f64, t5256: f64) -> f64 {
    let t10577 = t10576 * t306;
    let t10580 = t1322 * t9836;
    let t10584 = -1.6457779058161184_f64 * t5035 + 1.6457779058161184_f64 * t5082 + 3.600163427964126_f64 * t5085 + 7.35994946043302_f64 * t5087 - t5090 + 3.7610742193750633_f64 * t5092 + t5095 - 1.8805371096875316_f64 * t5104 - 1.2536914064583544_f64 * t5106 - t5115 + t5119 + t5121 + 1.8805371096875316_f64 * t5124 - 2.2140749178833072_f64 * t10577 * t311 + 1.6457779058161184_f64 * t10580 + 0.7380249726277691_f64 * t5225 - 0.7380249726277691_f64 * t5235 + t5256;
    t10584
}
