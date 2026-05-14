//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1188/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1188<F: Float>(t13043: F, t17525: F, t17526: F, t17529: F, t17532: F, t17535: F, t17538: F, t17540: F, t17543: F, t17546: F, t17547: F, t17549: F, t17551: F, t17553: F, t17555: F, t17559: F, t17560: F) -> (F, F) {
    let t17561 = 8.0 / 45.0 * t13043;
    let t17562 = t17525 + t17526 - t17529 + t17532 - t17535 + t17538 + t17540 + t17543 - t17546 - t17547 - t17549 + t17551 - t17553 - t17555 + t17559 - t17560 - t17561;
    (t17561, t17562)
}
