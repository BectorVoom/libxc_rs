//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 935/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk935(t395: f64, t9739: f64, t403: f64, t1458: f64, t2488: f64, t1435: f64, t2484: f64, t2478: f64, t2494: f64, t2491: f64, t1504: f64, t2594: f64, t2650: f64, t311: f64, t4770: f64, t5016: f64, t5020: f64, t5023: f64, t5328: f64, t9885: f64, t9887: f64, t9890: f64, t9892: f64, t9894: f64, t9896: f64) -> f64 {
    let t9899 = t395 * t9739;
    let t9902 = t403 * t9739;
    let t9905 = t2488 * t1458;
    let t9907 = t2484 * t1435;
    let t9909 = t2478 * t1435;
    let t9911 = t2494 * t1435;
    let t9913 = t2491 * t1435;
    let t9915 = -0.04115066352984959_f64 * t5328 * t2650 - t4770 * t2594 - t5016 - t5020 + 22.07984838129906_f64 * t9885 + 22.07984838129906_f64 * t9887 - 5.40024514194619_f64 * t9890 - 5.40024514194619_f64 * t9892 - t9894 * t1504 + 18.635258017632964_f64 * t9896 * t311 - 2.2140749178833072_f64 * t9899 * t311 - t5023 - 2.427516195194328_f64 * t9902 * t311 + 0.013716887843283197_f64 * t9905 - 1.6457779058161184_f64 * t9907 - 6.211752672544321_f64 * t9909 - 0.6268457032291772_f64 * t9911 - 6.496391258193384_f64 * t9913;
    t9915
}
