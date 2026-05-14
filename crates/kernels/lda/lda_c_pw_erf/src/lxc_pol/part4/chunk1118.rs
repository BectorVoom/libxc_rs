//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1118/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1118<F: Float>(t2416: F, t925: F, t1948: F, t1953: F, t557: F, t10195: F, t10202: F, t10204: F, t10206: F, t10208: F, t10225: F, t13583: F, t13585: F, t13587: F, t13589: F, t2520: F, t933: F) -> (F, F, F, F) {
    let t16345 = t925 * t2416;
    let t16348 = t1953 * t557 * t1948;
    let t16358 = 0.03199259259259259 * t16345 - 0.2879333333333333 * t16348 + t10195 - 0.017777777777777778 * t13583 + 0.015996296296296297 * t13585 + 0.026660493827160493 * t13587 - 0.06398518518518519 * t13589 + 0.03950617283950617 * t10202 + 0.014814814814814815 * t10204 - 0.007407407407407408 * t10206 - 0.0024691358024691358 * t10208 + t10225;
    let t16365 = t933 * t2520;
    (t16345, t16348, t16358, t16365)
}
