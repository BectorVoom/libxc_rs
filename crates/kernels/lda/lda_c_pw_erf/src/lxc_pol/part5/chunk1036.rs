//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1036/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1036<F: Float>(t17436: F, t2497: F, t806: F, t1325: F, t494: F, t5289: F, t12765: F, t519: F, t542: F, t784: F, t1318: F, t2478: F, t5269: F, t593: F, t833: F, t21564: F, t21568: F, t21570: F, t21571: F, t21573: F, t21575: F, t21576: F, t21581: F) -> (F, F, F, F, F, F) {
    let t21582 = 8.0 / 15.0 * t17436;
    let t21583 = t2497 * t806;
    let t21587 = 8.0 / 5.0 * t1325 * t5289 * t21583 * t494;
    let t21591 = 12.0 / 5.0 * t519 * t12765 * t21583 * t542;
    let t21596 = 8.0 / 5.0 * t1325 * t5289 * t2497 * t784 * t542;
    let t21601 = 8.0 / 5.0 * t1318 * t5269 * t2478 * t833 * t593;
    let t21602 = -t21564 + t21568 + t21570 - t21571 + t21573 + t21575 + t21576 - t21581 - t21582 + t21587 - t21591 + t21596 + t21601;
    (t21582, t21587, t21591, t21596, t21601, t21602)
}
