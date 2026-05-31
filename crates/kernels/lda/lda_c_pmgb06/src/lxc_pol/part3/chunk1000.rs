//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1000/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1000<F: Float>(t9352: F, t9379: F, t9381: F, t9383: F, t9385: F, t9393: F, t9395: F, t132: F, t1547: F, t2065: F, t1381: F, t1601: F, t2088: F, t5068: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11889 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t9352;
    let t11890 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t9379;
    let t11891 = F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t9381;
    let t11892 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t9383;
    let t11893 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9385;
    let t11894 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t9393;
    let t11895 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t9395;
    let t11897 = t132 * t1547 * t2065;
    let t11898 = t11897 / F::cast_from(45.0_f64);
    let t11902 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5068 * t1601 * t2088 * t1381;
    (t11889, t11890, t11891, t11892, t11893, t11894, t11895, t11898, t11902)
}
