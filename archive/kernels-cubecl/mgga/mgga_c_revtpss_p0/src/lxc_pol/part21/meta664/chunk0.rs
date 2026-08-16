//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2460/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2460<F: Float>(t11671: F, t3278: F, t12020: F, t3168: F, t2434: F, t246: F, t1041: F, t1046: F, t11256: F, t11258: F, t3172: F, t11727: F, t3188: F) -> (F, F, F, F, F, F) {
    let t42967 = t3278 * t11671;
    let t42970 = t12020 * t3168;
    let t42994 = t246 * t2434;
    let t42996 = t1041 * t42994 * t1046;
    let t43003 = t11256 * t3172 * t11258;
    let t43017 = t3188 * t11727;
    (t42967, t42970, t42994, t42996, t43003, t43017)
}
