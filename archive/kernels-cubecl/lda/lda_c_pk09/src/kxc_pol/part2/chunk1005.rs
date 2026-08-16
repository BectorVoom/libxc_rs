//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1005/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1005<F: Float>(t10869: F, t10885: F, t1381: F, t306: F, t395: F, t9602: F, t1426: F, t2487: F, t1368: F, t1287: F, t10025: F, t10116: F, t10392: F, t10395: F, t10854: F, t1292: F, t1406: F, t1427: F, t1450: F, t2587: F, t311: F, t5639: F, t5641: F, t5643: F, t5654: F, t5659: F, t5670: F, t5672: F, t5677: F) -> F {
    let t10886 = t10869 + t10885;
    let t10887 = t10886 * t1381;
    let t10888 = t10887 * t306;
    let t10891 = t395 * t9602;
    let t10894 = t1426 * t2487;
    let t10897 = t1368 * t9602;
    let t10898 = t10897 * t1287;
    let t10915 = F::cast_from(4.937333717448355_f64) * t10854 * t311 - F::cast_from(2.427516195194328_f64) * t10888 * t311 + F::cast_from(2.2140749178833072_f64) * t10891 * t1292 - F::cast_from(2.2140749178833072_f64) * t10894 * t311 - F::cast_from(18.635258017632964_f64) * t10898 - F::cast_from(10.80049028389238_f64) * t10392 + F::cast_from(10.80049028389238_f64) * t10395 + F::cast_from(1.2536914064583544_f64) * t5639 + F::cast_from(0.6268457032291772_f64) * t5641 - F::cast_from(4.4281498357666145_f64) * t1406 * t10025 + F::cast_from(2.2140749178833072_f64) * t1427 * t2587 - F::cast_from(2.9824072957409817_f64) * t1450 * t10116 - F::cast_from(2.427516195194328_f64) * t5643 - F::cast_from(3.7610742193750633_f64) * t5654 + F::cast_from(2.2140749178833072_f64) * t5659 + F::cast_from(0.6268457032291772_f64) * t5670 - F::cast_from(0.6268457032291772_f64) * t5672 + t5677;
    t10915
}
