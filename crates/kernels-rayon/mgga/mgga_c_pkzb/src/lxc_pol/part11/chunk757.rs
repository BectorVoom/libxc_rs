//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 757/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk757(t6087: f64, t2463: f64, t418: f64, t2411: f64, t300: f64, t1478: f64, t154: f64, t386: f64, t385: f64, t465: f64, t931: f64, t179: f64, t824: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6348 = 0.53272592592592592592e-1_f64 * t6087;
    let t6362 = 1.0_f64 / t2463 / t418;
    let t6366 = t300 * t2411;
    let t6377 = t154 * t1478 * t386;
    let t6379 = 5.0_f64 / 1296.0_f64 * t385 * t6377;
    let t6380 = t465 * t931;
    let t6382 = t179 * t6380 * t824;
    (t6348, t6362, t6366, t6377, t6379, t6380, t6382)
}
