//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1140/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1140<F: Float>(t1313: F, t2098: F, t2433: F, t519: F, t1472: F, t7746: F, t21014: F, t21015: F, t21016: F, t21019: F, t21022: F, t21025: F, t21028: F, t21032: F, t21033: F, t21034: F, t21036: F, t21038: F) -> (F, F, F) {
    let t21042 = F::new(8.0) / F::new(15.0) * t519 * t1313 * t2433 * t2098;
    let t21044 = F::new(8.0) / F::new(15.0) * t1472 * t7746;
    let t21045 = -t21014 + t21015 - t21016 - t21019 + t21022 + t21025 - t21028 - t21032 + t21033 - t21034 - t21036 - t21038 + t21042 + t21044;
    (t21042, t21044, t21045)
}
