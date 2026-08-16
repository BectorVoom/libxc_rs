//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1078/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1078(t4680: f64, t7575: f64, t9669: f64, t1181: f64, t5549: f64, t604: f64, t5544: f64, t1849: f64, t1992: f64, t30154: f64, t7586: f64, t30219: f64, t9653: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38946 = t7575 * t4680 * t9669;
    let t38950 = t7575 * t1181 * t604 * t5549;
    let t38954 = t7575 * t1181 * t604 * t5544;
    let t38956 = t1992 * t1849;
    let t38958 = t30154 * t7586 * t38956;
    let t38960 = t30219 * t9653;
    (t38946, t38950, t38954, t38956, t38958, t38960)
}
