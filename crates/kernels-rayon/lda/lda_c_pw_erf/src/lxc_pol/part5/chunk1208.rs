//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1208/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1208(t11866: f64, t13657: f64, t20813: f64, t13643: f64, t10043: f64, t10090: f64, t13710: f64, t13731: f64, t13736: f64, t14140: f64, t16345: f64, t16374: f64, t16397: f64, t16399: f64, t16432: f64, t16434: f64, t21839: f64, t21841: f64, t21843: f64, t21845: f64, t21849: f64, t21852: f64) -> (f64, f64, f64) {
    let t21855 = t11866 * t13657 * t20813;
    let t21858 = t11866 * t13643 * t20813;
    let t21866 = -0.005037777777777778_f64 * t16345 - t10043 - 0.0018891666666666666_f64 * t16374 + 0.002518888888888889_f64 * t16397 + 0.0016792592592592592_f64 * t16399 + 0.002518888888888889_f64 * t21839 - 0.003778333333333333_f64 * t21841 - 0.0006996913580246914_f64 * t21843 - 0.0006297222222222223_f64 * t21845 + 0.0018891666666666666_f64 * t21849 - 0.02267_f64 * t21852 + 0.006297222222222222_f64 * t21855 + 0.034005_f64 * t21858 - 0.0019591358024691357_f64 * t10090 - 0.011335_f64 * t16432 - 0.015113333333333333_f64 * t16434 - 0.005037777777777778_f64 * t13710 + t14140 - 0.005877407407407408_f64 * t13731 - 0.005037777777777778_f64 * t13736;
    (t21855, t21858, t21866)
}
