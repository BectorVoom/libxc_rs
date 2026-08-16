//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2121/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2121(t1404: f64, t8110: f64, t1851: f64, t7426: f64, t27907: f64, t580: f64, t2169: f64, t5381: f64, t1395: f64, t8119: f64, t1858: f64, t7415: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96283 = 2.0_f64 * t8110 * t1404;
    let t96285 = 2.0_f64 * t1851 * t7426;
    let t96289 = 2.0_f64 * t27907 * t580;
    let t96291 = 2.0_f64 * t2169 * t5381;
    let t96300 = 2.0_f64 * t1395 * t8119;
    let t96303 = 2.0_f64 * t7415 * t1858;
    (t96283, t96285, t96289, t96291, t96300, t96303)
}
