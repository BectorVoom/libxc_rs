//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1088/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1088(t19179: f64, t3792: f64, t30167: f64, t33202: f64, t11997: f64, t2778: f64, t11937: f64, t11781: f64, t9999: f64, t16182: f64, t29033: f64, t11483: f64, t928: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33464 = t3792 * t19179;
    let t33466 = t33202 * t30167;
    let t33468 = t11997 * t2778;
    let t33470 = t11937 * t2778;
    let t33472 = t11781 * t9999;
    let t33474 = t29033 * t16182;
    let t33476 = t928 * t11483;
    (t33464, t33466, t33468, t33470, t33472, t33474, t33476)
}
