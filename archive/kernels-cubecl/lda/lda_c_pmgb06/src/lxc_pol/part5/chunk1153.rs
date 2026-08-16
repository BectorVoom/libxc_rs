//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1153/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1153<F: Float>(t2466: F, t5305: F, t1972: F, t6541: F, t6545: F, t17666: F, t17668: F, t13707: F, t20843: F, t20845: F, t20847: F, t20849: F, t20852: F) -> (F, F, F, F, F, F) {
    let t20854 = t5305 * t2466 / F::cast_from(15.0_f64);
    let t20856 = t1972 * t6541 / F::cast_from(15.0_f64);
    let t20858 = t1972 * t6545 / F::cast_from(15.0_f64);
    let t20859 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t17666;
    let t20860 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t17668;
    let t20861 = -t20843 - t20845 - t20847 - t20849 + t13707 + t20852 + t20854 + t20856 + t20858 + t20859 - t20860;
    (t20854, t20856, t20858, t20859, t20860, t20861)
}
