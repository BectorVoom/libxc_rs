//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1002/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1002(t10823: f64, t10825: f64, t10827: f64, t10829: f64, t10841: f64, t10843: f64, t10846: f64, t10848: f64, t1345: f64, t2513: f64, t311: f64, t5227: f64, t5343: f64, t5593: f64, t5595: f64, t5606: f64, t5611: f64, t5627: f64, t5635: f64, t5637: f64, t9770: f64) -> f64 {
    let t10852 = 0.7380249726277691_f64 * t10823 + 1.2536914064583544_f64 * t10825 + 0.6268457032291772_f64 * t10827 - 1.2536914064583544_f64 * t10829 + 2.2140749178833072_f64 * t5227 * t2513 - 18.635258017632964_f64 * t5343 * t2513 - 18.635258017632964_f64 * t1345 * t9770 + t5593 + 2.427516195194328_f64 * t5595 + 14.216351496367702_f64 * t5606 - 14.216351496367702_f64 * t5611 + 3.5540878740919255_f64 * t5627 + 4.738783832122567_f64 * t10841 - 2.427516195194328_f64 * t10843 * t311 - 6.496391258193384_f64 * t10846 - 0.6268457032291772_f64 * t10848 + 0.013716887843283197_f64 * t5635 - 6.211752672544321_f64 * t5637;
    t10852
}
