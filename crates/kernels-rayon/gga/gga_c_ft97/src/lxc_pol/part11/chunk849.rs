//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 849/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk849(t1589: f64, t1636: f64, t89: f64, t375: f64, t8184: f64, t7752: f64, t23: f64, t32075: f64, t1588: f64, t27: f64, t7837: f64, t7999: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37421 = t89 * t1636 * t1589;
    let t37422 = 8.0_f64 / 9.0_f64 * t37421;
    let t37424 = t89 * t375 * t8184;
    let t37427 = t89 * t375 * t7752;
    let t37429 = t23 * t32075;
    let t37430 = t1588 * t1588;
    let t37433 = t89 * t27 * t37429 * t37430;
    let t37435 = t7837 * t7999;
    (t37421, t37422, t37424, t37427, t37430, t37433, t37435)
}
