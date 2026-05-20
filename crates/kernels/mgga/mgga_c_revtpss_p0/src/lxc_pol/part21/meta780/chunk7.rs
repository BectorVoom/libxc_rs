//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2788/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2788<F: Float>(t14979: F, t689: F, t779: F, t11044: F, t14983: F, t14485: F, t15014: F, t9303: F, t10510: F, t14987: F, t14991: F, t41066: F) -> (F, F, F, F, F, F) {
    let t51227 = t689 * t779 * t14979;
    let t51231 = t11044 * t14983;
    let t51233 = t11044 * t14485;
    let t51234 = F::cast_from(0.39029762157531132076e-1_f64) * t51233;
    let t51237 = t9303 * t15014;
    let t51239 = t14987 * t10510;
    let t51240 = F::cast_from(0.39029762157531132076e-1_f64) * t51239;
    let t51241 = t41066 * t14991;
    (t51227, t51231, t51234, t51237, t51240, t51241)
}
