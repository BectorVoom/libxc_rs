//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 580/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk580<F: Float>(t138: F, t1711: F, t1861: F, t2630: F, t2634: F, t2642: F, t444: F, t774: F, t101: F) -> (F, F) {
    let t2644 = t2630 * t138 + 2.0 * t1711 * t2634 - 2.0 * t1861 * t774 - t444 * t2642;
    let t2645 = t101 * t2644;
    (t2644, t2645)
}
