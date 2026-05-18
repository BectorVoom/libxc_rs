//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 799/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk799<F: Float>(t159: F, t285: F, t7337: F, t2700: F, t2703: F, t2709: F, t2712: F, t2739: F, t7323: F, t7324: F, t7325: F, t7326: F, t7327: F, t7328: F, t7329: F) -> (F, F) {
    let t7339 = t7337 * t159 * t285;
    let t7349 = -t7323 + t2700 + t2703 - t2709 - t2712 + t7324 - t7325 - t7326 - t2739 + t7327 + t7328 + t7329;
    (t7339, t7349)
}
