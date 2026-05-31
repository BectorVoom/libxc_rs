//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 648/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk648<F: Float>(t3833: F, t3889: F, t258: F, t248: F, t1092: F, t643: F, t1090: F, t638: F, t3736: F, t3744: F, t3746: F, t3748: F, t3762: F, t3764: F, t3766: F, t3768: F, t3867: F, t3871: F) -> (F, F, F, F, F, F) {
    let t3890 = t3833 + t3889;
    let t3891 = t258 * t3890;
    let t3892 = t248 * t3891;
    let t3893 = t643 * t1092;
    let t3895 = t638 * t1090;
    let t3897 = -t3736 - t3744 - F::cast_from(1.7544670867903938_f64) * t3746 - F::cast_from(51.94757731704439_f64) * t3748 - t3762 - t3764 + t3766 + F::cast_from(3.0_f64) * t3768 + t3892 - t3867 + t3871 - F::cast_from(24.0_f64) * t3893 + F::cast_from(12.0_f64) * t3895;
    (t3890, t3891, t3892, t3893, t3895, t3897)
}
