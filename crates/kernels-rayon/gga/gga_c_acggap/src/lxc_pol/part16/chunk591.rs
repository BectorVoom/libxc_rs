//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 591/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk591(t1937: f64, t322: f64, t449: f64, t316: f64, t1308: f64, t1614: f64, t1220: f64, t1914: f64, t119: f64, t1907: f64, t4137: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5510 = t1937 * t322;
    let t5511 = t449 * t5510;
    let t5512 = t316 * t5511;
    let t5514 = t1308 * t1614;
    let t5517 = t1220 * t1914 * t322;
    let t5518 = t316 * t5517;
    let t5520 = t119 * t1907;
    let t5523 = t4137 * t557;
    (t5511, t5512, t5514, t5517, t5518, t5520, t5523)
}
