//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 471/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk471<F: Float>(t138: F, t1706: F, t1711: F, t1859: F, t1861: F, t1864: F, t1878: F, t444: F, t450: F, t774: F, t101: F) -> (F, F) {
    let t1880 = t1859 * t138 - t1706 * t774 + F::cast_from(2.0_f64) * t1711 * t1864 - t1861 * t450 - t444 * t1878;
    let t1881 = t101 * t1880;
    (t1880, t1881)
}
