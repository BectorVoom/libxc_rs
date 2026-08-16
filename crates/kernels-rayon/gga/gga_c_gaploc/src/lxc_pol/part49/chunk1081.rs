//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1081/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1081(t13729: f64, t6313: f64, t13732: f64, t6305: f64, t12000: f64, t555: f64, t2268: f64, t888: f64, t11977: f64, t2349: f64, t42709: f64, t42712: f64, t42715: f64, t42718: f64, t42719: f64, t42722: f64, t42726: f64) -> (f64, f64) {
    let t46961 = t6313 * t13729;
    let t46963 = t6305 * t13732;
    let t46965 = t555 * t12000;
    let t46967 = t2268 * t46965 * t888;
    let t46970 = t2268 * t11977 * t2349;
    let t46973 = 0.37940008847568199465e-1_f64 * t42709 + t42712 + t42715 + t42718 + t42719 + t42722 + 0.7588001769513639893e-1_f64 * t46961 - 0.85365019907028448797e-1_f64 * t46963 - 0.85365019907028448797e-1_f64 * t46967 - 0.85365019907028448797e-1_f64 * t46970 + 0.15808337019820083111e-2_f64 * t42726;
    (t46965, t46973)
}
