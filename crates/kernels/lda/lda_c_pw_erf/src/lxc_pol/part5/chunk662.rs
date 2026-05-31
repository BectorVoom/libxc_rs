//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 662/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk662<F: Float>(t5592: F, t5594: F, t156: F, t1840: F, t426: F, t415: F, t763: F, t1859: F, t443: F, t1710: F, t770: F, t155: F, t436: F) -> (F, F, F, F, F, F, F) {
    let t5596 = F::cast_from(5.87616_f64) * t5592 * t5594;
    let t5598 = t426 * t156 * t1840;
    let t5607 = t415 * t763;
    let t5609 = F::cast_from(1.9486833333333333_f64) * t5607 * t5594;
    let t5618 = t1859 * t443;
    let t5621 = t770 * t1710;
    let t5639 = t155 * t436;
    (t5596, t5598, t5607, t5609, t5618, t5621, t5639)
}
