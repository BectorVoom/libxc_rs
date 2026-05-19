//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 882/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk882<F: Float>(t2268: F, t2304: F, t34273: F, t39849: F, t12803: F, t29874: F, t31586: F, t4261: F, t9074: F, t1063: F, t2854: F, t29969: F, t6320: F) -> (F, F, F, F, F) {
    let t42844 = F::cast_from(0.39837009289946609438e0_f64) * t2268 * t2304 * t34273;
    let t42845 = F::cast_from(0.142275033178380748e-1_f64) * t39849;
    let t42846 = t29874 * t12803;
    let t42847 = F::cast_from(0.47425011059460249332e-2_f64) * t42846;
    let t42849 = t9074 * t4261 * t31586;
    let t42850 = F::cast_from(0.47425011059460249332e-2_f64) * t42849;
    let t42857 = F::cast_from(0.17073003981405689759e0_f64) * t1063 * t6320 * t2854 * t29969;
    (t42844, t42845, t42847, t42850, t42857)
}
