//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 775/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk775<F: Float>(t1052: F, t1076: F, t1095: F, t2305: F, t2341: F, t2355: F, t3142: F, t3335: F, t3342: F, t4283: F, t4335: F, t8096: F, t8101: F, t9128: F, t9131: F, t9134: F, t9136: F, t9151: F, t98: F) -> (F,) {
    let t9153 = t4283 + t2305 * t4335 / 36.0 - t9128 * t98 / 6.0 - t9131 / 27.0 - t9134 / 6.0 + t9136 / 9.0 + 0.10237773105191754 * t3335 + 0.06825182070127836 * t3342 + t1076 * t8096 / 6.0 + t1076 * t8101 / 6.0 - t1095 * t2341 / 6.0 + t1052 * t8096 / 6.0 + t2355 * t3142 / 6.0 - t9151 / 6.0;
    (t9153,)
}
