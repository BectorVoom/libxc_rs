//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1313/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1313<F: Float>(t13243: F, t13245: F, t13249: F, t13251: F, t10087: F, t10089: F, t1444: F, t6752: F, t13182: F, t176: F, t1821: F, t493: F) -> (F, F, F, F, F, F, F, F) {
    let t17261 = F::cast_from(8.0_f64) / F::cast_from(243.0_f64) * t13243;
    let t17262 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13245;
    let t17263 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13249;
    let t17264 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13251;
    let t17265 = t10087 / F::cast_from(135.0_f64);
    let t17266 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t10089;
    let t17268 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1444 * t6752;
    let t17272 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t493 * t13182 * t176 * t1821;
    (t17261, t17262, t17263, t17264, t17265, t17266, t17268, t17272)
}
