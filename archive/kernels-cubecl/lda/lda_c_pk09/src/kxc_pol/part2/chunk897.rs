//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 897/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk897<F: Float>(t2149: F, t4093: F, t903: F, t115: F, t9204: F, t1052: F, t1059: F, t1101: F, t2341: F, t4085: F, t4489: F, t7706: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F, t8101: F, t8975: F) -> F {
    let t9492 = t903 * t4093 * t2149;
    let t9505 = t115 * t9204;
    let t9511 = -t1101 * t7706 / F::cast_from(6.0_f64) - t4085 * t9492 / F::cast_from(36.0_f64) - t4489 / F::cast_from(6.0_f64) - F::cast_from(0.10237773105191754_f64) * t7919 - F::cast_from(0.10237773105191754_f64) * t7923 - F::cast_from(0.10237773105191754_f64) * t7926 - F::cast_from(0.10237773105191754_f64) * t7928 - F::cast_from(0.10237773105191754_f64) * t7931 - F::cast_from(0.10237773105191754_f64) * t7935 - F::cast_from(0.06825182070127836_f64) * t7939 - F::cast_from(0.06825182070127836_f64) * t7942 - F::cast_from(0.09983749558483038_f64) * t8975 - t9505 / F::cast_from(9.0_f64) + t1052 * t8101 / F::cast_from(6.0_f64) - t1059 * t2341 / F::cast_from(6.0_f64);
    t9511
}
