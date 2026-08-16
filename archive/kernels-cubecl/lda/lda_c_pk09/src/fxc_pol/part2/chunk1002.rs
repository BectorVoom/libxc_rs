//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1002/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1002<F: Float>(t10823: F, t10825: F, t10827: F, t10829: F, t10841: F, t10843: F, t10846: F, t10848: F, t1345: F, t2513: F, t311: F, t5227: F, t5343: F, t5593: F, t5595: F, t5606: F, t5611: F, t5627: F, t5635: F, t5637: F, t9770: F) -> F {
    let t10852 = F::cast_from(0.7380249726277691_f64) * t10823 + F::cast_from(1.2536914064583544_f64) * t10825 + F::cast_from(0.6268457032291772_f64) * t10827 - F::cast_from(1.2536914064583544_f64) * t10829 + F::cast_from(2.2140749178833072_f64) * t5227 * t2513 - F::cast_from(18.635258017632964_f64) * t5343 * t2513 - F::cast_from(18.635258017632964_f64) * t1345 * t9770 + t5593 + F::cast_from(2.427516195194328_f64) * t5595 + F::cast_from(14.216351496367702_f64) * t5606 - F::cast_from(14.216351496367702_f64) * t5611 + F::cast_from(3.5540878740919255_f64) * t5627 + F::cast_from(4.738783832122567_f64) * t10841 - F::cast_from(2.427516195194328_f64) * t10843 * t311 - F::cast_from(6.496391258193384_f64) * t10846 - F::cast_from(0.6268457032291772_f64) * t10848 + F::cast_from(0.013716887843283197_f64) * t5635 - F::cast_from(6.211752672544321_f64) * t5637;
    t10852
}
