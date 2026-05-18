//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1453/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1453<F: Float>(t1227: F, t1234: F, t18609: F, t18616: F, t18622: F, t18625: F, t18628: F, t18630: F, t2712: F, t2715: F, t342: F, t35: F, t360: F, t4394: F, t6989: F, t6996: F, t7018: F, t780: F, t8263: F) -> F {
    let t18632 = -F::new(6.0) * t360 * t35 * t6996 * t1234 + F::new(30.0) * t360 * t35 * t6989 * t1234 - F::new(6.0) * t360 * t35 * t2712 * t1227 + F::new(3.0) * t360 * t35 * t780 * t4394 - t18609 + F::new(3.0) / F::new(2.0) * t360 * t35 * t2715 * t1227 - t18616 + F::new(3.0) * t360 * t35 * t7018 * t342 + F::new(4.0) * t18622 - F::new(2.0) * t18625 + F::new(29.3808) * t18628 - F::new(11.75232) * t18630 + t8263;
    t18632
}
