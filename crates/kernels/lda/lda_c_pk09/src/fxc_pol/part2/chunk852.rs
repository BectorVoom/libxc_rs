//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 852/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk852<F: Float>(t3317: F, t3319: F, t3335: F, t3342: F, t3384: F, t3388: F, t3393: F, t3629: F, t3632: F, t3633: F, t3634: F, t7851: F, t7855: F) -> F {
    let t8785 = F::cast_from(9.625452574583042_f64) * t7851 + F::cast_from(9.625452574583042_f64) * t7855 - F::new(0.64) * t3335 - F::cast_from(0.4266666666666667_f64) * t3342 + F::cast_from(19.250905149166083_f64) * t3384 + F::cast_from(19.250905149166083_f64) * t3388 - F::cast_from(19.250905149166083_f64) * t3393 + t3629 + t3632 + t3633 - t3634 + F::new(0.64) * t3317 + F::new(0.64) * t3319;
    t8785
}
