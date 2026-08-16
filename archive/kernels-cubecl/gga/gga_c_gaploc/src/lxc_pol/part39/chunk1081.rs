//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1081/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1081<F: Float>(t13729: F, t6313: F, t13732: F, t6305: F, t12000: F, t555: F, t2268: F, t888: F, t11977: F, t2349: F, t42709: F, t42712: F, t42715: F, t42718: F, t42719: F, t42722: F, t42726: F) -> (F, F) {
    let t46961 = t6313 * t13729;
    let t46963 = t6305 * t13732;
    let t46965 = t555 * t12000;
    let t46967 = t2268 * t46965 * t888;
    let t46970 = t2268 * t11977 * t2349;
    let t46973 = F::cast_from(0.37940008847568199465e-1_f64) * t42709 + t42712 + t42715 + t42718 + t42719 + t42722 + F::cast_from(0.7588001769513639893e-1_f64) * t46961 - F::cast_from(0.85365019907028448797e-1_f64) * t46963 - F::cast_from(0.85365019907028448797e-1_f64) * t46967 - F::cast_from(0.85365019907028448797e-1_f64) * t46970 + F::cast_from(0.15808337019820083111e-2_f64) * t42726;
    (t46965, t46973)
}
