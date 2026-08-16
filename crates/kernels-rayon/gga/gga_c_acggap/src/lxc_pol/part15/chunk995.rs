//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 995/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk995(t7433: f64, t8970: f64, t1181: f64, t22040: f64, t604: f64, t7493: f64, t21118: f64, t7351: f64, t7426: f64, t1165: f64, t21955: f64, t30806: f64) -> (f64, f64, f64, f64) {
    let t35092 = t7433 * t8970;
    let t35096 = t7493 * t1181 * t604 * t22040;
    let t35100 = t7426 * t1181 * t7351 * t21118;
    let t35113 = t30806 * t1165 * t604 * t21955;
    (t35092, t35096, t35100, t35113)
}
