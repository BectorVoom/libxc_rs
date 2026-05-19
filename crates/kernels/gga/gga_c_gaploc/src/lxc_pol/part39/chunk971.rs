//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 971/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk971<F: Float>(t42717: F, t39731: F, t2321: F, t34600: F, t9074: F, t12820: F, t484: F, t1063: F, t31308: F, t7937: F, t2268: F, t31399: F) -> (F, F, F, F, F, F) {
    let t42718 = F::cast_from(0.47425011059460249332e-2_f64) * t42717;
    let t42719 = F::cast_from(0.23712505529730124666e-2_f64) * t39731;
    let t42721 = t9074 * t34600 * t2321;
    let t42722 = F::cast_from(0.23712505529730124666e-2_f64) * t42721;
    let t42726 = t484 * t12820;
    let t42730 = F::cast_from(0.34146007962811379518e0_f64) * t1063 * t7937 * t31308;
    let t42733 = F::cast_from(0.68292015925622759036e0_f64) * t2268 * t7937 * t31399;
    (t42718, t42719, t42722, t42726, t42730, t42733)
}
