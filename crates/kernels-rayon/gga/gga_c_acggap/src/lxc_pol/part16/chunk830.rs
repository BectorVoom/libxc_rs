//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 830/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk830(t1713: f64, t599: f64, t142: f64, t7450: f64, t2313: f64, t507: f64, t2030: f64, t2317: f64, t2060: f64, t1849: f64, t604: f64, t1181: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9659 = t599 * t1713;
    let t9660 = t142 * t9659;
    let t9661 = t7450 * t9660;
    let t9663 = t507 * t2313;
    let t9664 = t2030 * t9663;
    let t9666 = t507 * t2317;
    let t9667 = t2060 * t9666;
    let t9669 = t604 * t1849;
    let t9670 = t1181 * t9669;
    (t9659, t9660, t9661, t9663, t9664, t9666, t9667, t9669, t9670)
}
