//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 687/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk687<F: Float>(t571: F, t6705: F, t2143: F, t2171: F, t4489: F, t504: F, t739: F, t806: F, t542: F, t348: F, t4494: F, t4501: F, t743: F, t833: F, t3976: F, t549: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6706 = t571 * t6705;
    let t6708 = t2171 * t2143;
    let t6710 = t4489 * t504;
    let t6711 = t739 * t806;
    let t6713 = t6710 * t6711 * t542;
    let t6716 = t6711 * t348;
    let t6717 = t4494 * t6716;
    let t6720 = t4501 * t6716;
    let t6723 = t743 * t833;
    let t6725 = t3976 * t6723 * t549;
    (t6706, t6708, t6710, t6711, t6713, t6717, t6720, t6723, t6725)
}
