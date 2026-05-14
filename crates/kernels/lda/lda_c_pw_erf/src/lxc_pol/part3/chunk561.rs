//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 561/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk561<F: Float>(t138: F, t1706: F, t1711: F, t1712: F, t1724: F, t3327: F, t3329: F, t3332: F, t3339: F, t3340: F, t3343: F, t3363: F, t444: F, t450: F, t101: F, t153: F, t274: F, t2869: F) -> (F, F, F) {
    let t3365 = t3327 * t138 - 3.0 * t1706 * t1724 + 6.0 * t1711 * t3343 + 6.0 * t3332 * t1712 - 3.0 * t3329 * t450 - 6.0 * t3339 * t3340 - t444 * t3363;
    let t3366 = t101 * t3365;
    let t3373 = 4.429070076315393 * t153 * t2869 * t274;
    (t3365, t3366, t3373)
}
