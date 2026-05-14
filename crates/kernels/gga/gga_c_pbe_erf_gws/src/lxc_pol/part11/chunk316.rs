//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 316/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk316<F: Float>(t1134: F, t858: F, t867: F, t866: F, t1105: F, t886: F, t884: F, t1127: F, t882: F, t339: F) -> (F, F, F, F, F, F, F) {
    let t1135 = t858 * t1134;
    let t1136 = t867 * t1135;
    let t1138 = t866 * t1136 / 96.0;
    let t1139 = t858 * t1105;
    let t1140 = t886 * t1139;
    let t1142 = t884 * t1140 / 48.0;
    let t1143 = t1127 - t1138 - t882 - t1142;
    let t1144 = t339 * t1143;
    (t1136, t1138, t1139, t1140, t1142, t1143, t1144)
}
