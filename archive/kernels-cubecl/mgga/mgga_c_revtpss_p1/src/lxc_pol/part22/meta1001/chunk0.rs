//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3406/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3406<F: Float>(t4707: F, t3011: F, t3014: F, t981: F, t11108: F, t6396: F, t2874: F, t63657: F, t935: F, t19471: F, t3022: F, t15534: F, t4719: F) -> (F, F, F, F, F, F) {
    let t63902 = t4707 * t4707;
    let t63906 = F::cast_from(0.34631718211362927518e2_f64) * t981 * t3011 * t63902 * t3014;
    let t63907 = t6396 * t11108;
    let t63916 = F::cast_from(4.0_f64) * t2874 * t63657 * t935;
    let t63918 = F::cast_from(0.69263436422725855036e2_f64) * t3022 * t19471;
    let t63920 = F::cast_from(0.11696447245269292414e1_f64) * t4719 * t15534;
    (t63902, t63906, t63907, t63916, t63918, t63920)
}
