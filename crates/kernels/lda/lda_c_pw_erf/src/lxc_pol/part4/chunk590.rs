//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 590/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk590<F: Float>(t2751: F, t174: F, t936: F, t998: F, t155: F, t912: F, t914: F, t1035: F, t344: F, t1553: F, t452: F, t405: F, t137: F, t142: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2752 = 0.07123333333333333 * t2751;
    let t2754 = t174 * t998 * t936;
    let t2755 = 0.053425 * t2754;
    let t2758 = t174 * t155 * t912 * t914;
    let t2759 = 0.10685 * t2758;
    let t2760 = t344 * t1035;
    let t2761 = 12.0 * t2760;
    let t2763 = t452 * t1553;
    let t2764 = t405 * t2763;
    let t2765 = t137 * t142;
    (t2752, t2754, t2755, t2758, t2759, t2760, t2761, t2763, t2764, t2765)
}
