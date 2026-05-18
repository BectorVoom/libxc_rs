//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1101/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1101<F: Float>(t133: F, t20430: F, t20427: F, t14585: F, t14641: F, t14652: F, t19773: F, t19775: F, t19782: F, t20340: F, t20341: F, t20342: F, t20345: F, t20353: F) -> F {
    let t20516 = t133 * t20430;
    let t20518 = t133 * t20427;
    let t20525 = -F::new(2.2990066666666666) * t14585 + F::new(6.89702) * t20516 + F::new(0.5747516666666667) * t20518 - F::new(1.724255) * t133 * t20345 - t14641 + t14652 + t20340 - t20341 - t20342 - F::new(2.2990066666666666) * t19773 + F::new(6.89702) * t19775 + F::new(1.724255) * t19782 + t20353;
    t20525
}
