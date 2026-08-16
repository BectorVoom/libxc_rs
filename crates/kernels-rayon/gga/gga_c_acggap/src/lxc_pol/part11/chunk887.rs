//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 887/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk887(t7433: f64, t7832: f64, t7423: f64, t30216: f64, t7588: f64, t30374: f64, t7428: f64, t121: f64, t413: f64, t168: f64, t1170: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30522 = t7433 * t7832;
    let t30524 = t7433 * t7423;
    let t30534 = t30216 * t7588;
    let t30536 = t30374 * t7428;
    let t30538 = t121 * t413;
    let t30539 = t30538 * t168;
    let t30540 = t1170 * t30539;
    (t30522, t30524, t30534, t30536, t30538, t30539, t30540)
}
