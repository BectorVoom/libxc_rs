//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1257/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1257<F: Float>(t13308: F, t16527: F, t5077: F, t12677: F, t493: F, t5318: F, t6119: F, t486: F, t6610: F, t5115: F, t802: F, t16505: F, t16507: F, t16510: F, t16512: F, t16516: F, t16518: F, t16521: F, t16523: F, t16525: F) -> (F, F, F, F, F, F) {
    let t16530 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t5077 * t13308 * t16527;
    let t16531 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t12677;
    let t16534 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t6119 * t5318;
    let t16535 = t486 * t6610;
    let t16536 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t16535;
    let t16537 = t802 * t5115;
    let t16538 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t16537;
    let t16539 = -t16505 + t16507 - t16510 - t16512 - t16516 - t16518 - t16521 + t16523 + t16525 - t16530 - t16531 + t16534 - t16536 - t16538;
    (t16530, t16531, t16534, t16536, t16538, t16539)
}
