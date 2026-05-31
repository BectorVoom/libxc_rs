//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 963/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk963<F: Float>(t11898: F, t2130: F, t493: F, t10162: F, t1325: F, t2182: F, t108: F, t2113: F, t267: F, t2010: F, t571: F, t9313: F) -> (F, F, F, F) {
    let t12987 = t493 * t11898 * t2130;
    let t12998 = t1325 * t10162 * t2182;
    let t12999 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t12998;
    let t13035 = t2113 * t108 * t267;
    let t13048 = t571 * t9313 * t2010;
    (t12987, t12999, t13035, t13048)
}
