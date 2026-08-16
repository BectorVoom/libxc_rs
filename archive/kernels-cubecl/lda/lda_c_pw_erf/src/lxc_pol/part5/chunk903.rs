//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 903/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk903<F: Float>(t3309: F, t436: F, t2: F, t39: F, t411: F, t120: F, t3318: F, t1: F, t119: F, t2824: F, t125: F, t2715: F, t3310: F) -> (F, F, F, F, F, F) {
    let t8896 = t3309 * t436;
    let t8898 = t2 * t39 * t411;
    let t8899 = t8896 * t8898;
    let t8901 = t3318 * t120;
    let t8902 = t8901 * t8898;
    let t8930 = t2824 * t1 * t119;
    let t8932 = F::cast_from(0.16322666666666666_f64) * t125 * t2715 * t3310 * t8930;
    (t8896, t8899, t8901, t8902, t8930, t8932)
}
