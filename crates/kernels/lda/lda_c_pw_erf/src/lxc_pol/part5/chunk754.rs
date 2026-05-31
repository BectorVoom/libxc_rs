//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 754/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk754<F: Float>(t644: F, t6887: F, t647: F, t2260: F, t3912: F, t256: F, t5236: F, t5240: F, t5284: F, t5304: F, t6858: F, t6860: F, t6862: F, t6864: F, t6869: F, t6871: F, t6873: F, t6877: F, t6879: F, t6881: F, t6885: F) -> (F, F, F, F) {
    let t6888 = t6887 * t644;
    let t6889 = t6888 * t647;
    let t6892 = F::cast_from(2e-21_f64) * t2260 * t3912;
    let t6893 = -t6858 - t6860 + t6862 + t6864 + t6869 - t6871 - t6873 - t6877 + t6879 + t6881 * t256 / F::cast_from(3.0_f64) + t6885 / F::cast_from(3.0_f64) + F::cast_from(0.06077777777777778_f64) * t6889 + t6892 - t5236 + t5240 + t5284 - t5304;
    (t6888, t6889, t6892, t6893)
}
