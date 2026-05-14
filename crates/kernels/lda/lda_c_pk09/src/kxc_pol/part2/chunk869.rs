//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 869/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk869<F: Float>(t10555: F, t10571: F, t10533: F, t10535: F, t10540: F, t1397: F, t1417: F, t2621: F, t392: F, t5139: F, t5144: F, t1425: F, t306: F, t1322: F, t9836: F, t311: F, t5035: F, t5082: F, t5085: F, t5087: F, t5090: F, t5092: F, t5095: F, t5104: F, t5106: F, t5115: F, t5119: F, t5121: F, t5124: F, t5225: F, t5235: F, t5256: F) -> (F, F) {
    let t10572 = t10555 + t10571;
    let t10575 = t10533 * t392 - t10535 * t1397 / 2.0 - t5139 * t2621 / 2.0 + 3.0 / 4.0 * t5144 * t10540 - t1417 * t10572 / 2.0;
    let t10576 = t10575 * t1425;
    let t10577 = t10576 * t306;
    let t10580 = t1322 * t9836;
    let t10584 = -1.6457779058161184 * t5035 + 1.6457779058161184 * t5082 + 3.600163427964126 * t5085 + 7.35994946043302 * t5087 - t5090 + 3.7610742193750633 * t5092 + t5095 - 1.8805371096875316 * t5104 - 1.2536914064583544 * t5106 - t5115 + t5119 + t5121 + 1.8805371096875316 * t5124 - 2.2140749178833072 * t10577 * t311 + 1.6457779058161184 * t10580 + 0.7380249726277691 * t5225 - 0.7380249726277691 * t5235 + t5256;
    (t10572, t10584)
}
