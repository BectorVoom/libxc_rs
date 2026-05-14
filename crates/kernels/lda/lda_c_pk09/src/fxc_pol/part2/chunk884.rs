//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 884/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk884<F: Float>(t10869: F, t10885: F, t1381: F, t306: F, t395: F, t9602: F, t1426: F, t2487: F, t1368: F, t1287: F, t10025: F, t10116: F, t10392: F, t10395: F, t10854: F, t1292: F, t1406: F, t1427: F, t1450: F, t2587: F, t311: F, t5639: F, t5641: F, t5643: F, t5654: F, t5659: F, t5670: F, t5672: F, t5677: F) -> (F,) {
    let t10886 = t10869 + t10885;
    let t10887 = t10886 * t1381;
    let t10888 = t10887 * t306;
    let t10891 = t395 * t9602;
    let t10894 = t1426 * t2487;
    let t10897 = t1368 * t9602;
    let t10898 = t10897 * t1287;
    let t10915 = 4.937333717448355 * t10854 * t311 - 2.427516195194328 * t10888 * t311 + 2.2140749178833072 * t10891 * t1292 - 2.2140749178833072 * t10894 * t311 - 18.635258017632964 * t10898 - 10.80049028389238 * t10392 + 10.80049028389238 * t10395 + 1.2536914064583544 * t5639 + 0.6268457032291772 * t5641 - 4.4281498357666145 * t1406 * t10025 + 2.2140749178833072 * t1427 * t2587 - 2.9824072957409817 * t1450 * t10116 - 2.427516195194328 * t5643 - 3.7610742193750633 * t5654 + 2.2140749178833072 * t5659 + 0.6268457032291772 * t5670 - 0.6268457032291772 * t5672 + t5677;
    (t10915,)
}
