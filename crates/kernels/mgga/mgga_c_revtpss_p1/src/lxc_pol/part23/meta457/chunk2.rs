//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1894/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1894<F: Float>(t19620: F, t6271: F, t3117: F, t19501: F, t3095: F, t3092: F, t1043: F, t3155: F) -> (F, F, F, F, F) {
    let t19621 = t6271 * t19620;
    let t19622 = t3117 * t19621;
    let t19625 = t19501 * t3095;
    let t19626 = t3092 * t19625;
    let t19634 = t3155 * t1043;
    (t19621, t19622, t19625, t19626, t19634)
}
