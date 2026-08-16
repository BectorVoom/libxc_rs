//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 952/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk952<F: Float>(t20750: F, t8392: F, t20769: F, t1882: F, t20916: F, t160: F, t20660: F, t20694: F, t20690: F, t20760: F, t20853: F, t376: F, t89: F) -> (F, F, F, F, F, F, F, F) {
    let t77642 = t8392 * t20750;
    let t77644 = t8392 * t20769;
    let t77678 = t1882 * t20916;
    let t77713 = t160 * t20660;
    let t77719 = t1882 * t20694;
    let t77721 = t1882 * t20690;
    let t77752 = t8392 * t20760;
    let t77821 = t89 * t376 * t20853;
    (t77642, t77644, t77678, t77713, t77719, t77721, t77752, t77821)
}
