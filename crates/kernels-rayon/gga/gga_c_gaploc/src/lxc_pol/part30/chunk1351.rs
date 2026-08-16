//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1351/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1351(t188: f64, t31793: f64, t3377: f64, t8158: f64, t9333: f64, t1508: f64, t2765: f64, t524: f64, t7930: f64, t8155: f64, t1572: f64, t16251: f64, t3354: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34143 = 0.10725146985555128001e1_f64 * t188 * t31793 * t3377;
    let t34145 = 0.21450293971110256002e1_f64 * t8158 * t9333;
    let t34148 = 0.10725146985555128001e1_f64 * t1508 * t2765 * t3377;
    let t34151 = 0.21450293971110256002e1_f64 * t524 * t7930 * t3377;
    let t34153 = 0.21450293971110256002e1_f64 * t8155 * t9333;
    let t34156 = 0.15889106645266856297e0_f64 * t1572 * t16251 * t3354;
    (t34143, t34145, t34148, t34151, t34153, t34156)
}
