//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1330/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1330<F: Float>(t230: F, t7827: F, t10278: F, t10286: F, t13377: F, t13380: F, t21698: F, t21700: F, t21703: F, t21706: F, t21711: F, t21713: F, t21714: F, t21717: F) -> F {
    let t23266 = t7827 * t230;
    let t23268 = F::cast_from(12.0_f64) * t13377 + t13380 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t10278 + t10286 + t21698 + t21700 + t21703 + t21706 - t21711 - t21713 - t21714 - t21717 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t23266;
    t23268
}
