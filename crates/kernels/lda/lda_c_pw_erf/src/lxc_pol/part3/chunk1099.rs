//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1099/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1099<F: Float>(t1310: F, t5334: F, t1472: F, t4770: F, t3802: F, t519: F, t5243: F, t10463: F, t1972: F, t12829: F, t12832: F, t12836: F, t12839: F, t12842: F, t12844: F, t12846: F, t12848: F, t12853: F) -> (F, F, F, F, F) {
    let t12855 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t5334 * t1310;
    let t12857 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1472 * t4770;
    let t12859 = t519 * t3802 * t5243;
    let t12860 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t12859;
    let t12862 = t519 * t10463 * t1972;
    let t12863 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t12862;
    let t12864 = -t12829 + t12832 - t12836 + t12839 + t12842 + t12844 - t12846 - t12848 - t12853 - t12855 - t12857 - t12860 + t12863;
    (t12855, t12857, t12860, t12863, t12864)
}
