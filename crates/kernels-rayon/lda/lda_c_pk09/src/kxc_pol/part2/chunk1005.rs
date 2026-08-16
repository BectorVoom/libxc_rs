//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1005/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1005(t10869: f64, t10885: f64, t1381: f64, t306: f64, t395: f64, t9602: f64, t1426: f64, t2487: f64, t1368: f64, t1287: f64, t10025: f64, t10116: f64, t10392: f64, t10395: f64, t10854: f64, t1292: f64, t1406: f64, t1427: f64, t1450: f64, t2587: f64, t311: f64, t5639: f64, t5641: f64, t5643: f64, t5654: f64, t5659: f64, t5670: f64, t5672: f64, t5677: f64) -> f64 {
    let t10886 = t10869 + t10885;
    let t10887 = t10886 * t1381;
    let t10888 = t10887 * t306;
    let t10891 = t395 * t9602;
    let t10894 = t1426 * t2487;
    let t10897 = t1368 * t9602;
    let t10898 = t10897 * t1287;
    let t10915 = 4.937333717448355_f64 * t10854 * t311 - 2.427516195194328_f64 * t10888 * t311 + 2.2140749178833072_f64 * t10891 * t1292 - 2.2140749178833072_f64 * t10894 * t311 - 18.635258017632964_f64 * t10898 - 10.80049028389238_f64 * t10392 + 10.80049028389238_f64 * t10395 + 1.2536914064583544_f64 * t5639 + 0.6268457032291772_f64 * t5641 - 4.4281498357666145_f64 * t1406 * t10025 + 2.2140749178833072_f64 * t1427 * t2587 - 2.9824072957409817_f64 * t1450 * t10116 - 2.427516195194328_f64 * t5643 - 3.7610742193750633_f64 * t5654 + 2.2140749178833072_f64 * t5659 + 0.6268457032291772_f64 * t5670 - 0.6268457032291772_f64 * t5672 + t5677;
    t10915
}
