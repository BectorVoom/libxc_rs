//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 882/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk882<F: Float>(t9501: F, t139: F, t3247: F, t1463: F, t1413: F, t1830: F, t508: F, t132: F, t2851: F, t478: F, t175: F, t3456: F) -> (F, F, F, F, F, F, F) {
    let t9502 = F::cast_from(0.01959135802469136_f64) * t9501;
    let t9507 = t139 * t3247;
    let t9508 = t1463 * t1463;
    let t9509 = F::cast_from(1.0_f64) / t9508;
    let t9525 = F::cast_from(1.0_f64) / t1463 / t1413;
    let t9552 = t1830 * t508;
    let t9596 = t132 * t2851 * t478;
    let t9636 = F::cast_from(1.0_f64) / t3456 / t175;
    (t9502, t9507, t9509, t9525, t9552, t9596, t9636)
}
