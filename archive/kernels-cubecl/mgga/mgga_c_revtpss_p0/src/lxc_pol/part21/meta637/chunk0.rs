//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2410/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2410<F: Float>(t11010: F, t689: F, t779: F, t2769: F, t786: F, t861: F, t10997: F, t11007: F, t252: F, t11009: F, t123: F, t676: F) -> (F, F, F, F, F) {
    let t41063 = t689 * t779 * t11010;
    let t41066 = t786 * t861 * t2769;
    let t41067 = t41066 * t10997;
    let t41070 = t786 * t252 * t11007;
    let t41073 = t41070 * t123 * t676 * t11009;
    (t41063, t41066, t41067, t41070, t41073)
}
