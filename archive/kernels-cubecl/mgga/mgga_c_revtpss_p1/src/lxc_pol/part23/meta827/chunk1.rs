//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2684/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2684<F: Float>(t11922: F, t16081: F, t19749: F, t20020: F, t3211: F, t15656: F, t4845: F, t19675: F, t372: F, t11947: F, t20016: F, t11875: F, t19757: F) -> (F, F, F, F, F, F) {
    let t67025 = t16081 * t11922 * t19749;
    let t67044 = t3211 * t20020;
    let t67048 = t15656 * t4845;
    let t67052 = t372 * t19675;
    let t67072 = t11947 * t20016;
    let t67152 = t11875 * t11922 * t19757;
    (t67025, t67044, t67048, t67052, t67072, t67152)
}
