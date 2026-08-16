//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1216/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1216(t3262: f64, t3574: f64, t41202: f64, t12045: f64, t40282: f64, t12060: f64, t40713: f64, t3275: f64, t3465: f64, t42940: f64, t39030: f64, t40630: f64, t43771: f64) -> (f64, f64, f64, f64, f64) {
    let t44147 = 3.0_f64 / 2.0_f64 * t3262 * t41202 * t3574;
    let t44150 = 3.0_f64 / 2.0_f64 * t40282 * t12045;
    let t44152 = 5.0_f64 / 8.0_f64 * t40713 * t12060;
    let t44155 = t3275 * t3465 * t42940 / 2.0_f64;
    let t44158 = 3.0_f64 * t40630 * t39030 * t43771;
    (t44147, t44150, t44152, t44155, t44158)
}
