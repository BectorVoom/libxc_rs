//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1208/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1208<F: Float>(t11866: F, t13657: F, t20813: F, t13643: F, t10043: F, t10090: F, t13710: F, t13731: F, t13736: F, t14140: F, t16345: F, t16374: F, t16397: F, t16399: F, t16432: F, t16434: F, t21839: F, t21841: F, t21843: F, t21845: F, t21849: F, t21852: F) -> (F, F, F) {
    let t21855 = t11866 * t13657 * t20813;
    let t21858 = t11866 * t13643 * t20813;
    let t21866 = -F::cast_from(0.005037777777777778_f64) * t16345 - t10043 - F::cast_from(0.0018891666666666666_f64) * t16374 + F::cast_from(0.002518888888888889_f64) * t16397 + F::cast_from(0.0016792592592592592_f64) * t16399 + F::cast_from(0.002518888888888889_f64) * t21839 - F::cast_from(0.003778333333333333_f64) * t21841 - F::cast_from(0.0006996913580246914_f64) * t21843 - F::cast_from(0.0006297222222222223_f64) * t21845 + F::cast_from(0.0018891666666666666_f64) * t21849 - F::new(0.02267) * t21852 + F::cast_from(0.006297222222222222_f64) * t21855 + F::new(0.034005) * t21858 - F::cast_from(0.0019591358024691357_f64) * t10090 - F::new(0.011335) * t16432 - F::cast_from(0.015113333333333333_f64) * t16434 - F::cast_from(0.005037777777777778_f64) * t13710 + t14140 - F::cast_from(0.005877407407407408_f64) * t13731 - F::cast_from(0.005037777777777778_f64) * t13736;
    (t21855, t21858, t21866)
}
