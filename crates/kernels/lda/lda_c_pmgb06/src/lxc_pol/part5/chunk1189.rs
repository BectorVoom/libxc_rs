//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1189/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1189<F: Float>(t2448: F, t27: F, t402: F, t5770: F, t5783: F, t110: F, t360: F, t7313: F, t18755: F, t18757: F, t18759: F, t2209: F, t2233: F, t2712: F, t342: F, t35: F, t5980: F, t7278: F, t7310: F, t780: F) -> (F, F) {
    let t21473 = t2448 * t27 * t402;
    let t21474 = t5770 * t21473;
    let t21476 = t5783 * t21473;
    let t21477 = F::cast_from(2.923025_f64) * t21476;
    let t21479 = t360 * t110 * t7313;
    let t21501 = F::cast_from(2.0_f64) * t18755 + F::cast_from(11.75232_f64) * t18757 - F::cast_from(2.93808_f64) * t18759 - F::cast_from(8.81424_f64) * t21474 - t21477 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t21479 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t360 * t35 * t2233 * t2448 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t360 * t35 * t780 * t5980 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t360 * t35 * t7310 * t342 + F::cast_from(30.0_f64) * t360 * t35 * t7278 * t342 - F::cast_from(18.0_f64) * t360 * t35 * t2712 * t2209;
    (t21477, t21501)
}
