//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 970/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk970<F: Float>(t133: F, t14585: F, t14641: F, t14652: F, t19773: F, t19775: F, t19782: F, t20340: F, t20341: F, t20342: F, t20345: F, t20353: F, t20516: F, t20518: F, t14684: F, t14692: F, t14698: F, t1870: F, t20294: F, t20356: F, t20374: F, t20390: F, t20396: F, t20403: F, t20406: F, t20409: F, t20412: F, t20433: F, t20434: F) -> (F, F) {
    let t20525 = -2.2990066666666666 * t14585 + 6.89702 * t20516 + 0.5747516666666667 * t20518 - 1.724255 * t133 * t20345 - t14641 + t14652 + t20340 - t20341 - t20342 - 2.2990066666666666 * t19773 + 6.89702 * t19775 + 1.724255 * t19782 + t20353;
    let t20529 = t20356 - 62.07318 * t1870 * t20294 - t20374 + t20390 + t20396 + t20403 - t20406 + t20409 + t20412 - t14684 - t20433 + t14692 + 6.89702 * t14698 + t20434;
    (t20525, t20529)
}
