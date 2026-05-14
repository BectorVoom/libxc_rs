//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 999/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk999<F: Float>(t13379: F, t230: F, t4714: F, t4521: F, t833: F, t3610: F, t4506: F, t211: F, t4567: F, t4575: F, t2127: F, t3455: F, t10278: F, t10286: F, t13364: F, t13367: F, t13371: F, t13373: F, t13376: F, t13377: F) -> (F, F, F, F, F) {
    let t13380 = 8.0 * t13379;
    let t13381 = t4714 * t230;
    let t13384 = t4521 * t833;
    let t13387 = 4.0 / 9.0 * t4506 * t13384 * t3610;
    let t13389 = t211 * t4567 * t4575;
    let t13390 = 32.0 / 45.0 * t13389;
    let t13391 = t3455 * t2127;
    let t13392 = 8.0 / 15.0 * t13391;
    let t13393 = -t13364 + t13367 - t13371 - t13373 + t13376 + 4.0 * t13377 + t13380 + 4.0 * t13381 + 4.0 * t10278 + t10286 - t13387 + t13390 + t13392;
    (t13384, t13387, t13390, t13392, t13393)
}
