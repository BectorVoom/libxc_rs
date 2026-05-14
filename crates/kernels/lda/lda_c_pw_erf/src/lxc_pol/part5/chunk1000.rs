//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1000/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1000<F: Float>(t16084: F, t16092: F, t4804: F, t7738: F, t2146: F, t6287: F, t1313: F, t2098: F, t2433: F, t519: F, t1472: F, t7746: F, t21014: F, t21015: F, t21016: F, t21019: F, t21022: F, t21025: F, t21028: F, t21032: F) -> (F, F, F, F, F, F, F) {
    let t21033 = 16.0 / 45.0 * t16084;
    let t21034 = 16.0 / 45.0 * t16092;
    let t21036 = 16.0 / 15.0 * t4804 * t7738;
    let t21038 = 4.0 / 15.0 * t2146 * t6287;
    let t21042 = 8.0 / 15.0 * t519 * t1313 * t2433 * t2098;
    let t21044 = 8.0 / 15.0 * t1472 * t7746;
    let t21045 = -t21014 + t21015 - t21016 - t21019 + t21022 + t21025 - t21028 - t21032 + t21033 - t21034 - t21036 - t21038 + t21042 + t21044;
    (t21033, t21034, t21036, t21038, t21042, t21044, t21045)
}
