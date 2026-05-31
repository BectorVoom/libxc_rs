//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 737/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk737<F: Float>(t11: F, t6662: F, t558: F, t6005: F, t557: F, t3627: F, t4013: F, t4657: F, t4659: F, t4662: F, t4663: F, t6638: F, t6641: F, t6644: F, t6647: F, t6649: F, t6652: F, t6655: F, t6657: F, t6660: F) -> (F, F, F, F, F) {
    let t6663 = t11 * t6662;
    let t6665 = t558 * t6005;
    let t6666 = t557 * t6665;
    let t6667 = t11 * t6666;
    let t6669 = t4013 + F::cast_from(0.0008396296296296296_f64) * t3627 + F::cast_from(0.0016792592592592592_f64) * t4657 - F::cast_from(0.0008396296296296296_f64) * t4659 + t4662 + F::cast_from(0.002518888888888889_f64) * t4663 - F::cast_from(0.0004198148148148148_f64) * t6638 + F::cast_from(0.002099074074074074_f64) * t6641 - F::cast_from(0.007556666666666666_f64) * t6644 - F::cast_from(0.005037777777777778_f64) * t6647 + F::cast_from(0.0012594444444444445_f64) * t6649 + F::cast_from(0.011335_f64) * t6652 + F::cast_from(0.015113333333333333_f64) * t6655 - F::cast_from(0.0006297222222222223_f64) * t6657 + F::cast_from(0.0012594444444444445_f64) * t6660 - F::cast_from(0.003778333333333333_f64) * t6663 + F::cast_from(0.0018891666666666666_f64) * t6667;
    (t6663, t6665, t6666, t6667, t6669)
}
