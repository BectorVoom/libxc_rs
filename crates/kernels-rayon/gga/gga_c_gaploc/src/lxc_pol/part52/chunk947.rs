//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 947/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk947(t12092: f64, t2482: f64, t9267: f64, t40009: f64, t40013: f64, t40015: f64, t40019: f64, t12000: f64, t123: f64, t883: f64, t2487: f64, t2488: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47869 = t9267 * t12092 * t2482;
    let t47871 = 0.63904876589867916128e-1_f64 * t40009;
    let t47873 = 0.63904876589867916128e-1_f64 * t40013;
    let t47874 = 0.63904876589867916128e-1_f64 * t40015;
    let t47875 = 0.63904876589867916128e-1_f64 * t40019;
    let t47877 = t12000 * t123 * t883;
    let t47879 = t2487 * t2488 * t47877;
    (t47869, t47871, t47873, t47874, t47875, t47877, t47879)
}
