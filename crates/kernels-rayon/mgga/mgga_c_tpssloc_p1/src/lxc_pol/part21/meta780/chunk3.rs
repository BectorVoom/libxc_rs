//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2709/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2709(t12283: f64, t19981: f64, t19986: f64, t16205: f64, t3792: f64, t19823: f64, t40021: f64, t12211: f64, t19827: f64, t19831: f64, t1351: f64, t6330: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57143 = t12283 * t19981;
    let t57145 = t12283 * t19986;
    let t57147 = t3792 * t16205;
    let t57158 = t40021 * t19823;
    let t57160 = t12211 * t19827;
    let t57170 = t12211 * t19831;
    let t57172 = t6330 * t1351;
    (t57143, t57145, t57147, t57158, t57160, t57170, t57172)
}
