//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 987/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk987<F: Float>(t10576: F, t306: F, t1322: F, t9836: F, t311: F, t5035: F, t5082: F, t5085: F, t5087: F, t5090: F, t5092: F, t5095: F, t5104: F, t5106: F, t5115: F, t5119: F, t5121: F, t5124: F, t5225: F, t5235: F, t5256: F) -> F {
    let t10577 = t10576 * t306;
    let t10580 = t1322 * t9836;
    let t10584 = -F::new(1.6457779058161184) * t5035 + F::new(1.6457779058161184) * t5082 + F::new(3.600163427964126) * t5085 + F::new(7.35994946043302) * t5087 - t5090 + F::new(3.7610742193750633) * t5092 + t5095 - F::new(1.8805371096875316) * t5104 - F::new(1.2536914064583544) * t5106 - t5115 + t5119 + t5121 + F::new(1.8805371096875316) * t5124 - F::new(2.2140749178833072) * t10577 * t311 + F::new(1.6457779058161184) * t10580 + F::new(0.7380249726277691) * t5225 - F::new(0.7380249726277691) * t5235 + t5256;
    t10584
}
