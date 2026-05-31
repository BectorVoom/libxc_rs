//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 787/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk787<F: Float>(t230: F, t2660: F, t4595: F, t4718: F, t4719: F, t6578: F, t6582: F, t6584: F, t6586: F, t6588: F, t6594: F, t6599: F, t6603: F, t6605: F, t6606: F, t6613: F, t6633: F, t6673: F) -> (F, F) {
    let t7256 = t2660 * t230;
    let t7258 = -t6578 + t6582 + t6584 + t6586 + t6588 + t6594 + t6599 + t6603 + t6605 - t6606 - t4595 + t6613 + t6633 + t6673 + t4718 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t4719 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t7256;
    (t7256, t7258)
}
