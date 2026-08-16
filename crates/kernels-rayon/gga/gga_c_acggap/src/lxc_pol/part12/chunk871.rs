//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 871/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk871(t1181: f64, t30209: f64, t3650: f64, t604: f64, t1170: f64, t2066: f64, t592: f64, t7634: f64, t7844: f64, t10098: f64, t7336: f64) -> (f64, f64, f64, f64) {
    let t30212 = t30209 * t1181 * t604 * t3650;
    let t30216 = t1170 * t592 * t7634 * t2066;
    let t30217 = t30216 * t7844;
    let t30219 = t10098 * t7336;
    (t30212, t30216, t30217, t30219)
}
