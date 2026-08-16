//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 884/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk884<F: Float>(t220: F, t7776: F, t211: F, t156: F, t5926: F, t670: F, t1999: F, t542: F, t196: F, t5174: F, t188: F, t10: F, t225: F, t5902: F) -> (F, F, F, F, F) {
    let t16488 = t7776 * t220;
    let t16490 = F::cast_from(112.0_f64) / F::cast_from(1215.0_f64) * t211 * t16488;
    let t16498 = F::cast_from(0.43284165449459373508e0_f64) * t670 * t156 * t5926;
    let t16501 = F::cast_from(0.38474813732852776452e0_f64) * t670 * t542 * t1999;
    let t16531 = F::cast_from(1.0_f64) / t5174 / t196;
    let t16532 = t188 * t16531;
    let t16553 = F::cast_from(0.43284165449459373508e0_f64) * t670 * t10 * t225 * t5902;
    (t16490, t16498, t16501, t16532, t16553)
}
