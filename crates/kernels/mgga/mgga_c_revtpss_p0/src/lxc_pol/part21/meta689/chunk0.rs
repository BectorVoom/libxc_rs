//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2509/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2509<F: Float>(t1222: F, t3688: F, t697: F, t13001: F, t140: F, t1226: F, t2438: F, t12855: F, t12857: F, t12916: F, t12956: F, t12959: F) -> (F, F, F, F, F) {
    let t44925 = t1222 * t697 * t3688;
    let t44928 = t1222 * t140 * t13001;
    let t44931 = t1222 * t2438 * t1226;
    let t44938 = t12855 * t12916 * t12857;
    let t44949 = t12956 * t12959;
    (t44925, t44928, t44931, t44938, t44949)
}
