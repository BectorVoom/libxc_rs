//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 907/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk907<F: Float>(t11065: F, t4481: F, t638: F, t8614: F, t1101: F, t2160: F, t2158: F, t2799: F, t898: F, t2801: F, t3947: F, t3952: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11066 = F::new(24.0) * t11065;
    let t11073 = F::new(24.0) * t638 * t4481;
    let t11083 = F::new(240.0) * t8614;
    let t11090 = t1101 * t2160;
    let t11092 = t1101 * t2158;
    let t11093 = F::new(60.0) * t11092;
    let t11095 = t2799 * t898;
    let t11097 = t2801 * t898;
    let t11098 = F::new(144.0) * t11097;
    let t11099 = t3947 * t898;
    let t11100 = F::new(240.0) * t11099;
    let t11101 = t3952 * t898;
    (t11066, t11073, t11083, t11090, t11093, t11095, t11098, t11100, t11101)
}
