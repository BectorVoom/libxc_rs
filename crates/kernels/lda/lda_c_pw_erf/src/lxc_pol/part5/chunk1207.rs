//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1207/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1207<F: Float>(t325: F, t7430: F, t7437: F, t7449: F, t7452: F, t20027: F, t558: F, t11: F, t557: F, t11866: F, t13653: F, t20813: F) -> (F, F, F, F, F, F, F) {
    let t21839 = t325 * t7430;
    let t21841 = t325 * t7437;
    let t21843 = t325 * t7449;
    let t21845 = t325 * t7452;
    let t21847 = t558 * t20027;
    let t21849 = t11 * t557 * t21847;
    let t21852 = t11866 * t13653 * t20813;
    (t21839, t21841, t21843, t21845, t21847, t21849, t21852)
}
