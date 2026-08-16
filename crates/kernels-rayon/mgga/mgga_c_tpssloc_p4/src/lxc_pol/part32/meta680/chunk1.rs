//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2120/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2120(t45844: f64, t7245: f64, t22550: f64, t7974: f64, t2109: f64, t90247: f64, t1419: f64, t2274: f64, t111: f64, t27370: f64, t2174: f64, t5363: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96120 = t45844 * t7245;
    let t96135 = t7974 * t22550;
    let t96138 = t2109 * t90247;
    let t96157 = t1419 * t2274;
    let t96238 = t27370 * t111;
    let t96281 = 2.0_f64 * t5363 * t2174;
    (t96120, t96135, t96138, t96157, t96238, t96281)
}
