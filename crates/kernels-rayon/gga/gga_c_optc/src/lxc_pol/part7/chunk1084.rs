//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1084/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1084(t2555: f64, t7207: f64, t280: f64, t881: f64, t355: f64, t7194: f64, t992: f64, t2435: f64, t2436: f64, t7244: f64, t8285: f64, t92: f64, t93: f64) -> (f64, f64, f64, f64, f64) {
    let t23495 = t2555 * t7207;
    let t23503 = 1.0_f64 / t280 / t881;
    let t23510 = t355 * t7194 * t992;
    let t23513 = t2435 * t2436 * t7244;
    let t23518 = 1.0_f64 / t8285 / t92 * t93;
    (t23495, t23503, t23510, t23513, t23518)
}
