//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1427/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1427(t35204: f64, t6734: f64, t204: f64, t34246: f64, t587: f64, t2413: f64, t26127: f64, t34239: f64, t6717: f64, t6914: f64, t10241: f64, t1359: f64) -> (f64, f64, f64, f64, f64) {
    let t35206 = 0.21450293971110256001e2_f64 * t35204 * t6734;
    let t35209 = 0.92023022289409799224e1_f64 * t587 * t204 * t34246;
    let t35211 = 0.21450293971110256002e1_f64 * t26127 * t2413;
    let t35214 = 0.12423108009070322895e3_f64 * t6914 * t6717 * t34239;
    let t35215 = t1359 * t10241;
    (t35206, t35209, t35211, t35214, t35215)
}
