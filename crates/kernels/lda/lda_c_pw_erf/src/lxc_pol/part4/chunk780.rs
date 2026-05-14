//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 780/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk780<F: Float>(t5592: F, t5594: F, t156: F, t1840: F, t426: F, t1856: F, t411: F, t1568: F, t767: F, t415: F, t763: F, t1664: F, t1852: F, t10: F, t127: F, t3313: F, t3322: F, t5588: F, t5591: F) -> (F, F, F, F, F, F, F, F) {
    let t5596 = 5.87616 * t5592 * t5594;
    let t5598 = t426 * t156 * t1840;
    let t5599 = t1856 * t411;
    let t5603 = t767 * t1568;
    let t5607 = t415 * t763;
    let t5609 = 1.9486833333333333 * t5607 * t5594;
    let t5610 = t1852 * t1664;
    let t5614 = 5.87616 * t127 * t1852 * t1568 + t5588 + t5591 - t5596 - t5598 + 3.0 * t426 * t10 * t5599 + 3.0 / 2.0 * t426 * t10 * t5603 - t5609 - 6.0 * t426 * t10 * t5610 + t3313 - t3322;
    (t5596, t5598, t5599, t5603, t5607, t5609, t5610, t5614)
}
