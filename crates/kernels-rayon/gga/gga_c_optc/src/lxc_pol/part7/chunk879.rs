//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 879/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk879(t6548: f64, t8428: f64, t8426: f64, t914: f64, t1027: f64, t6554: f64, t1221: f64, t2367: f64, t3280: f64, t1220: f64, t1135: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8429 = t8428 * t6548;
    let t8430 = t8426 * t8429;
    let t8431 = t914 * t8430;
    let t8434 = t1027 * t6554;
    let t8435 = t1221 * t8434;
    let t8436 = t914 * t8435;
    let t8443 = t2367 * t3280;
    let t8444 = t1220 * t8443;
    let t8446 = t9 * t1135;
    (t8429, t8430, t8431, t8434, t8435, t8436, t8443, t8444, t8446)
}
