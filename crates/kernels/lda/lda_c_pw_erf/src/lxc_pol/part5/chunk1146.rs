//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1146/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1146<F: Float>(t1997: F, t6988: F, t16221: F, t16224: F, t16232: F, t21083: F, t21085: F, t21087: F, t21089: F, t21091: F, t21093: F, t21096: F, t21099: F, t21104: F) -> (F, F, F, F, F) {
    let t21106 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t6988 * t1997;
    let t21107 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t16221;
    let t21108 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t16224;
    let t21109 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t16232;
    let t21110 = t21083 - t21085 + t21087 + t21089 + t21091 + t21093 - t21096 - t21099 + t21104 - t21106 - t21107 + t21108 - t21109;
    (t21106, t21107, t21108, t21109, t21110)
}
