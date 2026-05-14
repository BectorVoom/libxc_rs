//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1214/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1214<F: Float>(t12804: F, t16548: F, t16550: F, t16555: F, t16557: F, t16559: F, t16560: F, t16561: F, t16562: F, t16566: F, t16568: F, t16569: F, t16573: F, t16574: F, t18244: F, t16577: F, t16579: F, t16581: F, t16584: F, t16585: F, t16587: F, t16588: F, t16589: F, t16590: F, t16591: F, t16594: F, t16599: F, t16603: F, t9759: F, t9770: F) -> (F, F) {
    let t18247 = -t16548 + t16550 - t16555 + t16557 - t16559 + t16560 + 0.033245444444444446 * t18244 - t16561 - t16562 + t16566 + t16568 + 16.0 / 81.0 * t12804 - t16569 - t16573 - t16574;
    let t18248 = -t16577 - t16579 - t16581 - t16584 - t16585 + t9759 - t16587 - t16588 - t16589 + t16590 + t16591 - t16594 - t9770 - t16599 - t16603;
    (t18247, t18248)
}
