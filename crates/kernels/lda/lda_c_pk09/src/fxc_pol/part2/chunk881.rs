//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 881/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk881<F: Float>(t1435: F, t2475: F, t2497: F, t1342: F, t9836: F, t1307: F, t10186: F, t1476: F, t1215: F, t2689: F, t2629: F, t2667: F, t1345: F, t2513: F, t311: F, t5227: F, t5343: F, t5593: F, t5595: F, t5606: F, t5611: F, t5627: F, t5635: F, t5637: F, t9770: F) -> (F,) {
    let t10823 = t2475 * t1435;
    let t10825 = t2497 * t1435;
    let t10827 = t1342 * t9836;
    let t10829 = t1307 * t9836;
    let t10841 = t1476 * t10186;
    let t10843 = t2689 * t1215;
    let t10846 = t2629 * t1435;
    let t10848 = t2667 * t1435;
    let t10852 = 0.7380249726277691 * t10823 + 1.2536914064583544 * t10825 + 0.6268457032291772 * t10827 - 1.2536914064583544 * t10829 + 2.2140749178833072 * t5227 * t2513 - 18.635258017632964 * t5343 * t2513 - 18.635258017632964 * t1345 * t9770 + t5593 + 2.427516195194328 * t5595 + 14.216351496367702 * t5606 - 14.216351496367702 * t5611 + 3.5540878740919255 * t5627 + 4.738783832122567 * t10841 - 2.427516195194328 * t10843 * t311 - 6.496391258193384 * t10846 - 0.6268457032291772 * t10848 + 0.013716887843283197 * t5635 - 6.211752672544321 * t5637;
    (t10852,)
}
