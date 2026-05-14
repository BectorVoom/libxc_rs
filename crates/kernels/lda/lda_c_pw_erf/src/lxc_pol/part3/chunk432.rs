//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 432/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk432<F: Float>(t138: F, t1704: F, t1706: F, t1711: F, t1712: F, t1724: F, t444: F, t450: F, t101: F, t100: F, t95: F) -> (F, F, F) {
    let t1726 = t1704 * t138 - 2.0 * t1706 * t450 + 2.0 * t1711 * t1712 - t444 * t1724;
    let t1727 = t101 * t1726;
    let t1729 = t95 * t100;
    (t1726, t1727, t1729)
}
