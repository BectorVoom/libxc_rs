//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1001/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1001(t7433: f64, t8739: f64, t1089: f64, t2079: f64, t535: f64, t7542: f64, t1967: f64, t8978: f64, t31095: f64, t31100: f64, t33953: f64, t5127: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35260 = t7433 * t8739;
    let t35271 = t2079 * t1089 * t535 * t7542;
    let t35273 = t1967 * t8978;
    let t35278 = 0.17149607247227894789e-2_f64 * t31095;
    let t35279 = 0.42874018118069736972e-2_f64 * t31100;
    let t35284 = t33953 * t5127;
    (t35260, t35271, t35273, t35278, t35279, t35284)
}
