//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1103/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1103<F: Float>(t1706: F, t1711: F, t1861: F, t1878: F, t20507: F, t20513: F, t20525: F, t20529: F, t2634: F, t2642: F, t3332: F, t3339: F, t444: F, t450: F, t7211: F, t774: F, t7957: F, t7960: F, t7974: F, t9059: F, t9068: F) -> F {
    let t20533 = -F::new(3.0) * t1861 * t7211 - F::new(6.0) * t9059 * t7957 + F::new(24.0) * t9068 * t7957 * t450 - F::new(18.0) * t3339 * t2634 * t1878 + F::new(6.0) * t3332 * t7960 - F::new(18.0) * t3339 * t7960 * t450 + F::new(6.0) * t1711 * t1878 * t2642 + F::new(6.0) * t1711 * t774 * t7211 - t1706 * t7974 + F::new(2.0) * t1711 * t7974 * t450 - t444 * (t20507 + t20513 + t20525 + t20529);
    t20533
}
