//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 287/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk287<F: Float>(t453: F, t8: F, t6: F, t95: F, t1194: F, t1199: F, t1201: F, t408: F, t101: F, t1088: F, t1093: F, t1094: F, t1121: F, t1189: F, t387: F, t397: F, t400: F, t415: F, t72: F, t75: F) -> (F, F, F) {
    let t1204 = F::new(1.0) / t8 / t453;
    let t1205 = t6 * t1204;
    let t1206 = t95 * t1205;
    let t1208 = -F::cast_from(0.11955719325063177623e-1_f64) * t408 + F::new(0.40985e-2) * t1194 - F::cast_from(0.10566666666666666667e-2_f64) * t1199 + F::cast_from(0.3884654180847230157e-4_f64) * t1201 - F::cast_from(0.420109375e-5_f64) * t1206;
    let t1210 = F::cast_from(0.23426533963880895498e-2_f64) * t408 * t72 + F::cast_from(0.46853067927761790996e-2_f64) * t1088 * t397 + F::cast_from(0.70279601891642686494e-2_f64) * t1093 * t1094 - F::cast_from(0.23426533963880895498e-2_f64) * t387 * t1121 - t1189 * t101 - F::new(2.0) * t400 * t415 - t75 * t1208;
    (t1206, t1208, t1210)
}
