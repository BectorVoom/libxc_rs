//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1084/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1084<F: Float>(t14432: F, t14433: F, t14437: F, t20087: F, t20090: F, t20091: F, t20092: F, t20094: F, t8469: F, t8473: F, t8477: F, t8481: F, t8491: F, t8505: F, t8509: F, t8516: F) -> F {
    let t20198 = -t20087 - t20090 + t8469 + t8473 - t8477 - t8481 + t20091 - t20092 + t8491 + t14432 + t14433 - t8505 + t8509 + t20094 + t14437 + t8516;
    t20198
}
