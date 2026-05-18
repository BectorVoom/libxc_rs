//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 877/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk877<F: Float>(t1059: F, t2737: F, t2849: F, t391: F, t358: F, t3160: F, t2851: F, t3166: F, t3017: F, t339: F, t344: F, t935: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8189 = t1059 * t2737;
    let t8191 = t2849 * t391;
    let t8195 = t2849 * t358;
    let t8197 = t3160 * t391;
    let t8199 = t2851 * t358;
    let t8202 = F::new(480.0) * t3166 * t391;
    let t8204 = t3160 * t358;
    let t8206 = t339 * t3017;
    let t8212 = t3166 * t358;
    let t8216 = t344 * t3017;
    let t8218 = t935 * t935;
    (t8189, t8191, t8195, t8197, t8199, t8202, t8204, t8206, t8212, t8216, t8218)
}
