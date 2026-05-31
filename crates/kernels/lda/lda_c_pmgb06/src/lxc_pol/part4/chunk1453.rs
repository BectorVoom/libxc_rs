//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1453/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1453<F: Float>(t1227: F, t1234: F, t18609: F, t18616: F, t18622: F, t18625: F, t18628: F, t18630: F, t2712: F, t2715: F, t342: F, t35: F, t360: F, t4394: F, t6989: F, t6996: F, t7018: F, t780: F, t8263: F) -> F {
    let t18632 = -F::cast_from(6.0_f64) * t360 * t35 * t6996 * t1234 + F::cast_from(30.0_f64) * t360 * t35 * t6989 * t1234 - F::cast_from(6.0_f64) * t360 * t35 * t2712 * t1227 + F::cast_from(3.0_f64) * t360 * t35 * t780 * t4394 - t18609 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t360 * t35 * t2715 * t1227 - t18616 + F::cast_from(3.0_f64) * t360 * t35 * t7018 * t342 + F::cast_from(4.0_f64) * t18622 - F::cast_from(2.0_f64) * t18625 + F::cast_from(29.3808_f64) * t18628 - F::cast_from(11.75232_f64) * t18630 + t8263;
    t18632
}
