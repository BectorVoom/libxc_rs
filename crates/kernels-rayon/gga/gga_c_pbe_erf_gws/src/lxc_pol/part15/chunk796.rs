//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 796/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk796(t147: f64, t922: f64, t1378: f64, t285: f64, t799: f64, t1488: f64, t751: f64, t1492: f64, t1497: f64, t309: f64, t310: f64, t311: f64) -> (f64, f64, f64, f64, f64) {
    let t6054 = t922 * t147;
    let t6055 = t6054 * t1378;
    let t6056 = t799 * t285;
    let t6058 = 0.45692190944741466895e-5_f64 * t6055 * t6056;
    let t6059 = t751 * t1488;
    let t6061 = t751 * t1492;
    let t6064 = 0.59871170051273045469e-1_f64 * t751 * t1497;
    let t6072 = 1.0_f64 / t311 / t310 / t309;
    (t6058, t6059, t6061, t6064, t6072)
}
