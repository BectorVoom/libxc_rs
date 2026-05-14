//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1049/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1049<F: Float>(t325: F, t7430: F, t7437: F, t7449: F, t7452: F, t20027: F, t558: F, t11: F, t557: F, t11866: F, t13653: F, t20813: F, t13657: F, t13643: F, t10043: F, t10090: F, t13710: F, t13731: F, t13736: F, t14140: F, t16345: F, t16374: F, t16397: F, t16399: F, t16432: F, t16434: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21839 = t325 * t7430;
    let t21841 = t325 * t7437;
    let t21843 = t325 * t7449;
    let t21845 = t325 * t7452;
    let t21847 = t558 * t20027;
    let t21849 = t11 * t557 * t21847;
    let t21852 = t11866 * t13653 * t20813;
    let t21855 = t11866 * t13657 * t20813;
    let t21858 = t11866 * t13643 * t20813;
    let t21866 = -0.005037777777777778 * t16345 - t10043 - 0.0018891666666666666 * t16374 + 0.002518888888888889 * t16397 + 0.0016792592592592592 * t16399 + 0.002518888888888889 * t21839 - 0.003778333333333333 * t21841 - 0.0006996913580246914 * t21843 - 0.0006297222222222223 * t21845 + 0.0018891666666666666 * t21849 - 0.02267 * t21852 + 0.006297222222222222 * t21855 + 0.034005 * t21858 - 0.0019591358024691357 * t10090 - 0.011335 * t16432 - 0.015113333333333333 * t16434 - 0.005037777777777778 * t13710 + t14140 - 0.005877407407407408 * t13731 - 0.005037777777777778 * t13736;
    (t21839, t21841, t21843, t21845, t21847, t21849, t21852, t21855, t21858, t21866)
}
