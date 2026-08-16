//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1355/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1355(t35198: f64, t18970: f64, t3381: f64, t1: f64, t1559: f64, t544: f64, t986: f64, t6734: f64, t204: f64, t34246: f64, t587: f64, t2413: f64, t26127: f64) -> (f64, f64, f64, f64, f64) {
    let t35199 = 0.14896037479937677779e-1_f64 * t35198;
    let t35200 = t18970 * t3381;
    let t35201 = 0.14896037479937677779e-1_f64 * t35200;
    let t35204 = t544 * t1559 * t986 * t1;
    let t35206 = 0.21450293971110256001e2_f64 * t35204 * t6734;
    let t35209 = 0.92023022289409799224e1_f64 * t587 * t204 * t34246;
    let t35211 = 0.21450293971110256002e1_f64 * t26127 * t2413;
    (t35199, t35201, t35206, t35209, t35211)
}
