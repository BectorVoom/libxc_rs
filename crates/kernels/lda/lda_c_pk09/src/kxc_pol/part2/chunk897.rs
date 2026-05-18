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
    let t9511 = -t1101 * t7706 / F::new(6.0) - t4085 * t9492 / F::new(36.0) - t4489 / F::new(6.0) - F::new(0.10237773105191754) * t7919 - F::new(0.10237773105191754) * t7923 - F::new(0.10237773105191754) * t7926 - F::new(0.10237773105191754) * t7928 - F::new(0.10237773105191754) * t7931 - F::new(0.10237773105191754) * t7935 - F::new(0.06825182070127836) * t7939 - F::new(0.06825182070127836) * t7942 - F::new(0.09983749558483038) * t8975 - t9505 / F::new(9.0) + t1052 * t8101 / F::new(6.0) - t1059 * t2341 / F::new(6.0);
    t9511
}
