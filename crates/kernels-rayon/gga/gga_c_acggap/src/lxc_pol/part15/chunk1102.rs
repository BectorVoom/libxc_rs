//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1102/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1102(t1181: f64, t2068: f64, t25742: f64, t604: f64, t1165: f64, t26459: f64, t7337: f64, t7315: f64, t9622: f64, t2016: f64, t9626: f64, t5936: f64, t8511: f64) -> (f64, f64, f64, f64, f64) {
    let t39035 = t2068 * t1181 * t604 * t25742;
    let t39039 = t7337 * t1165 * t604 * t26459;
    let t39041 = t7315 * t9622;
    let t39043 = t2016 * t9626;
    let t39049 = t8511 * t5936;
    (t39035, t39039, t39041, t39043, t39049)
}
