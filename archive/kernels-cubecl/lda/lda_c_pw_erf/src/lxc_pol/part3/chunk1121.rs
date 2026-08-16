//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1121/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1121<F: Float>(t13115: F, t13117: F, t5166: F, t13079: F, t13083: F, t13085: F, t13087: F, t13092: F, t13096: F, t13098: F, t13100: F, t13103: F, t13106: F, t13110: F, t13114: F) -> (F, F) {
    let t13120 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t13115 * t5166 * t13117;
    let t13121 = t13079 + t13083 - t13085 - t13087 - t13092 + t13096 + t13098 + t13100 + t13103 + t13106 + t13110 + t13114 - t13120;
    (t13120, t13121)
}
