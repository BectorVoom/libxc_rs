//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 508/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk508<F: Float>(t1181: F, t3169: F, t388: F, t1163: F, t322: F, t435: F, t157: F, t372: F, t406: F) -> (F, F, F, F) {
    let t3171 = t1181 * t388 * t3169;
    let t3172 = t1163 * t3171;
    let t3174 = t435 * t322;
    let t3176 = t372 * t406 * t157;
    (t3171, t3172, t3174, t3176)
}
