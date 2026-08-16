//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 975/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk975<F: Float>(t11172: F, t1445: F, t2293: F, t597: F, t13383: F, t1580: F, t11259: F, t2464: F, t2465: F, t6914: F, t18313: F, t18372: F, t44386: F, t590: F) -> (F, F, F, F) {
    let t46471 = F::cast_from(0.43710935587469654631e2_f64) * t597 * t1445 * t11172 * t2293;
    let t46473 = F::cast_from(0.11502877786176224903e2_f64) * t1580 * t13383;
    let t46480 = t6914 * t2464 * t2465 * t11259;
    let t46490 = F::cast_from(0.61348681526273199482e1_f64) * t18372 * t18313 * t44386 * t590;
    (t46471, t46473, t46480, t46490)
}
