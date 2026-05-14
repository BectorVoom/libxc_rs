//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1242/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1242<F: Float>(t110: F, t360: F, t7031: F, t2707: F, t348: F, t5772: F, t7035: F, t7027: F, t365: F, t6989: F, t18588: F, t5770: F, t1227: F, t1234: F, t2712: F, t2715: F, t342: F, t35: F, t4394: F, t6996: F, t7018: F, t780: F, t8263: F) -> (F, F) {
    let t18609 = t360 * t110 * t7031;
    let t18615 = t348 * t2707 * t5772;
    let t18616 = 1.9486833333333333 * t18615;
    let t18622 = t360 * t110 * t7035;
    let t18625 = t360 * t110 * t7027;
    let t18628 = t365 * t6989 * t5772;
    let t18630 = t5770 * t18588;
    let t18632 = -6.0 * t360 * t35 * t6996 * t1234 + 30.0 * t360 * t35 * t6989 * t1234 - 6.0 * t360 * t35 * t2712 * t1227 + 3.0 * t360 * t35 * t780 * t4394 - t18609 + 3.0 / 2.0 * t360 * t35 * t2715 * t1227 - t18616 + 3.0 * t360 * t35 * t7018 * t342 + 4.0 * t18622 - 2.0 * t18625 + 29.3808 * t18628 - 11.75232 * t18630 + t8263;
    (t18616, t18632)
}
