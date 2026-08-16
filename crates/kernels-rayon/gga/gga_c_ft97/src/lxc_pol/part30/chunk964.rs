//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 964/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk964(t140959: f64, t33948: f64, t141052: f64, t28558: f64, t141002: f64, t7607: f64, t140943: f64, t33941: f64, t33942: f64, t140927: f64, t7006: f64, t33934: f64, t33935: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t142779 = t33948 * t140959;
    let t142787 = t28558 * t141052;
    let t142810 = 0.8891911659407557944e-2_f64 * t7607 * t141002;
    let t142815 = t33941 * t140943 * t33942;
    let t142818 = 0.20139801475612389137e-1_f64 * t7006 * t140927;
    let t142820 = t33934 * t140943 * t33935;
    (t142779, t142787, t142810, t142815, t142818, t142820)
}
