//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 642/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk642(t147: f64, t6045: f64, t551: f64, t1480: f64, t1473: f64, t759: f64, t922: f64, t1378: f64, t285: f64, t799: f64, t1497: f64, t751: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6046 = t6045 * t147;
    let t6047 = t6046 * t551;
    let t6049 = 0.16396719238543588599e-3_f64 * t6047 * t1480;
    let t6053 = 0.15965645347006145458e0_f64 * t1473 * t759;
    let t6054 = t922 * t147;
    let t6055 = t6054 * t1378;
    let t6056 = t799 * t285;
    let t6058 = 0.45692190944741466895e-5_f64 * t6055 * t6056;
    let t6064 = 0.59871170051273045469e-1_f64 * t751 * t1497;
    (t6046, t6047, t6049, t6053, t6054, t6055, t6056, t6058, t6064)
}
