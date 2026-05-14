//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 824/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk824<F: Float>(t1435: F, t2484: F, t2478: F, t2494: F, t2491: F, t1504: F, t2594: F, t2650: F, t311: F, t4770: F, t5016: F, t5020: F, t5023: F, t5328: F, t9885: F, t9887: F, t9890: F, t9892: F, t9894: F, t9896: F, t9899: F, t9902: F, t9905: F) -> (F,) {
    let t9907 = t2484 * t1435;
    let t9909 = t2478 * t1435;
    let t9911 = t2494 * t1435;
    let t9913 = t2491 * t1435;
    let t9915 = -0.04115066352984959 * t5328 * t2650 - t4770 * t2594 - t5016 - t5020 + 22.07984838129906 * t9885 + 22.07984838129906 * t9887 - 5.40024514194619 * t9890 - 5.40024514194619 * t9892 - t9894 * t1504 + 18.635258017632964 * t9896 * t311 - 2.2140749178833072 * t9899 * t311 - t5023 - 2.427516195194328 * t9902 * t311 + 0.013716887843283197 * t9905 - 1.6457779058161184 * t9907 - 6.211752672544321 * t9909 - 0.6268457032291772 * t9911 - 6.496391258193384 * t9913;
    (t9915,)
}
