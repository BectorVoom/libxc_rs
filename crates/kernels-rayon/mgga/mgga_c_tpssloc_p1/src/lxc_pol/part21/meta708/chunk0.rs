//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2541/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2541(t2841: f64, t4351: f64, t10701: f64, t1543: f64, t10810: f64, t1561: f64, t14363: f64, t942: f64, t2929: f64, t4446: f64, t1568: f64, t2886: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49269 = t4351 * t2841;
    let t49274 = t1543 * t10701;
    let t49285 = t1561 * t10810;
    let t49404 = t14363 * t942;
    let t49411 = t4446 * t2929;
    let t49422 = t2886 * t1568;
    (t49269, t49274, t49285, t49404, t49411, t49422)
}
