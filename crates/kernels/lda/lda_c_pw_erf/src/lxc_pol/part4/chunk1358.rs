//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1358/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1358<F: Float>(t443: F, t7166: F, t1878: F, t1710: F, t2630: F, t1664: F, t1870: F, t19503: F, t19517: F, t19519: F, t19521: F, t19558: F, t19563: F, t19564: F, t19565: F, t19566: F, t19567: F, t2610: F, t5651: F, t8865: F, t8869: F, t8873: F, t8936: F, t9083: F) -> (F, F, F, F) {
    let t19703 = t7166 * t443;
    let t19723 = t1878 * t1878;
    let t19726 = t2630 * t1710;
    let t19733 = t8865 - t8869 + t8873 + t19503 + t9083 + t19517 - t19519 + t19521 - t8936 + t19558 + t19563 - t19564 + t19565 - t19566 - t19567 - 20.69106 * t1870 * t5651 * t2610 * t1664;
    (t19703, t19723, t19726, t19733)
}
