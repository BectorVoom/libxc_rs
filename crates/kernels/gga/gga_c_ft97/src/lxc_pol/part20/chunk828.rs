//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 828/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk828<F: Float>(t6260: F, t875: F, t840: F, t871: F, t1476: F, t2801: F, t2749: F, t6287: F, t25000: F, t2862: F, t319: F, t6278: F, t882: F, t25004: F, t1882: F, t6280: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25220 = t6260 * t875;
    let t25222 = t840 * t871 * t25220;
    let t25225 = t1476 * t2801;
    let t25227 = t840 * t871 * t25225;
    let t25231 = t840 * t2749 * t6287;
    let t25235 = t2862 * t319 * t25000;
    let t25239 = t2862 * t882 * t6278;
    let t25243 = t2862 * t319 * t25004;
    let t25246 = t1882 * t6280;
    (t25220, t25222, t25225, t25227, t25231, t25235, t25239, t25243, t25246)
}
