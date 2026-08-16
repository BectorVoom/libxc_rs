//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 983/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk983<F: Float>(t1652: F, t1833: F, t933: F, t1870: F, t1872: F, t436: F, t473: F, t1814: F, t1953: F, t3338: F, t770: F, t1710: F, t1859: F) -> (F, F, F, F, F) {
    let t14691 = t1652 * t1833 * t933;
    let t14692 = F::cast_from(0.9743416666666667_f64) * t14691;
    let t14698 = t1870 * t473 * t436 * t1872;
    let t14718 = t1814 * t1953;
    let t14724 = t770 * t3338;
    let t14729 = t1859 * t1710;
    (t14692, t14698, t14718, t14724, t14729)
}
