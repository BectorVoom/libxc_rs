//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 159/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk159<F: Float>(t1131: F, t676: F, t27: F, t89: F, t1089: F, t664: F, t661: F, t1091: F, t265: F, t724: F, t729: F, t738: F, t992: F, t737: F, t743: F, t192: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1132 = t676 * t1131;
    let t1134 = t89 * t27 * t1132;
    let t1136 = -t664 - t1089 / 18.0 - t1134 / 6.0;
    let t1137 = t661 * t1136;
    let t1140 = t724 * t265 * t1091;
    let t1144 = t729 * t265 * t1131;
    let t1147 = t738 * t992;
    let t1148 = t737 * t1147;
    let t1151 = t743 * t1131;
    let t1152 = t192 * t1151;
    (t1132, t1134, t1136, t1137, t1140, t1144, t1147, t1148, t1152)
}
