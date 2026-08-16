//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 897/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk897(t2149: f64, t4093: f64, t903: f64, t115: f64, t9204: f64, t1052: f64, t1059: f64, t1101: f64, t2341: f64, t4085: f64, t4489: f64, t7706: f64, t7919: f64, t7923: f64, t7926: f64, t7928: f64, t7931: f64, t7935: f64, t7939: f64, t7942: f64, t8101: f64, t8975: f64) -> f64 {
    let t9492 = t903 * t4093 * t2149;
    let t9505 = t115 * t9204;
    let t9511 = -t1101 * t7706 / 6.0_f64 - t4085 * t9492 / 36.0_f64 - t4489 / 6.0_f64 - 0.10237773105191754_f64 * t7919 - 0.10237773105191754_f64 * t7923 - 0.10237773105191754_f64 * t7926 - 0.10237773105191754_f64 * t7928 - 0.10237773105191754_f64 * t7931 - 0.10237773105191754_f64 * t7935 - 0.06825182070127836_f64 * t7939 - 0.06825182070127836_f64 * t7942 - 0.09983749558483038_f64 * t8975 - t9505 / 9.0_f64 + t1052 * t8101 / 6.0_f64 - t1059 * t2341 / 6.0_f64;
    t9511
}
