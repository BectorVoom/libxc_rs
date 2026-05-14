//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 638/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk638<F: Float>(t13534: F, t13566: F, t11701: F, t977: F, t3459: F, t8862: F, t3638: F, t7324: F, t5559: F, t1052: F, t3511: F, t2592: F, t3684: F, t1960: F, t3601: F, t7290: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13567 = t13534 + t13566;
    let t13569 = t11701 * t977;
    let t13573 = 4.0 * t8862 * t3459;
    let t13577 = 2.0 * t7324 * t3638;
    let t13578 = t3638 * t977;
    let t13580 = 6.0 * t5559 * t13578;
    let t13581 = t1052 * t3511;
    let t13584 = t2592 * t3684;
    let t13585 = t3684 * t977;
    let t13587 = 2.0 * t1960 * t13585;
    let t13588 = t7290 * t3601;
    (t13567, t13569, t13573, t13577, t13578, t13580, t13581, t13584, t13585, t13587, t13588)
}
