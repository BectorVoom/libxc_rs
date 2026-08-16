//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1104/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1104<F: Float>(t161: F, t166: F, t2093: F, t6904: F, t1848: F, t2625: F, t6596: F, t831: F, t16687: F, t16689: F, t17719: F, t1924: F, t5068: F) -> (F, F, F, F, F, F) {
    let t20279 = t161 * t166 * t2093 * t6904 / F::cast_from(10.0_f64);
    let t20281 = t1848 * t2625 / F::cast_from(10.0_f64);
    let t20283 = t831 * t6596 / F::cast_from(10.0_f64);
    let t20284 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t16687;
    let t20285 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t16689;
    let t20288 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5068 * t17719 * t1924;
    (t20279, t20281, t20283, t20284, t20285, t20288)
}
