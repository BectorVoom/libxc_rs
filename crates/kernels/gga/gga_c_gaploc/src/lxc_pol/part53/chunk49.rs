//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 49/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk49<F: Float>(t122: F, t158: F, t169: F, t172: F, t105: F, t33: F, t58: F, t75: F, t110: F, t112: F, t22: F, t6: F, t101: F, t107: F, t119: F) -> (F, F, F, F, F, F, F, F) {
    let t174 = t122 * t158 * t169 * t172;
    let t177 = -t33 + t58 + 0.28455006635676149599e-1 * t105 * t174;
    let t178 = f64::sqrt(4.0);
    let t179 = t75 * t178;
    let t180 = t110 * t112;
    let t183 = t6 * t22;
    let t187 = 0.619125e-2 * t179 * t180 - 0.79593333333333333331e-1 * t107 * t183 * t101;
    let t188 = t187 * t119;
    (t174, t177, t178, t179, t180, t183, t187, t188)
}
