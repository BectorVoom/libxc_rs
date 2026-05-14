//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 287/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk287<F: Float>(t1091: F, t265: F, t724: F, t1131: F, t729: F, t738: F, t992: F, t737: F, t743: F, t192: F, t462: F, t736: F, t92: F, t734: F, t91: F, t1089: F, t1134: F, t751: F) -> (F, F, F, F, F, F, F, F) {
    let t1140 = t724 * t265 * t1091;
    let t1144 = t729 * t265 * t1131;
    let t1147 = t738 * t992;
    let t1148 = t737 * t1147;
    let t1151 = t743 * t1131;
    let t1152 = t192 * t1151;
    let t1154 = -t736 - t462 * t1148 / 3.0 - t92 * t1152;
    let t1156 = t91 * t734 * t1154;
    let t1160 = t1156 / 6.0 - t751 - t1089 / 9.0 - t1134 / 3.0;
    (t1140, t1144, t1147, t1148, t1152, t1154, t1156, t1160)
}
