//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 559/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk559<F: Float>(t2271: F, t766: F, t2054: F, t2056: F, t2049: F, t758: F, t757: F) -> (F, F, F, F) {
    let t2272 = t2271 * t766;
    let t2279 = t2054 * t2056;
    let t2281 = t758 * t2049;
    let t2287 = t757 * t757;
    (t2272, t2279, t2281, t2287)
}
