//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1209/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1209<F: Float>(t13507: F, t13511: F, t13515: F, t13517: F, t13523: F, t13538: F, t17900: F, t17902: F, t17904: F, t17907: F, t17909: F, t17911: F, t17912: F, t17913: F, t17914: F, t17915: F) -> (F, F, F, F, F, F, F) {
    let t17916 = 32.0 / 135.0 * t13507;
    let t17917 = 32.0 / 405.0 * t13511;
    let t17918 = 512.0 / 405.0 * t13515;
    let t17919 = 32.0 / 405.0 * t13517;
    let t17920 = 512.0 / 405.0 * t13523;
    let t17921 = 32.0 / 81.0 * t13538;
    let t17922 = -t17900 + t17902 + t17904 - t17907 + 0.011181742741110338 * t17909 + t17911 - t17912 - t17913 - t17914 - t17915 + t17916 - t17917 - t17918 - t17919 + t17920 - t17921;
    (t17916, t17917, t17918, t17919, t17920, t17921, t17922)
}
