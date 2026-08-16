//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1935/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1935(t26223: f64, t26364: f64, t26485: f64, t26500: f64, t533: f64, t1390: f64, t1983: f64, t16521: f64, t1873: f64, t16524: f64, t7015: f64, t5371: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26502 = t26223 + t26364 + t26485 + t26500;
    let t26503 = t533 * t26502;
    let t26504 = t26503 * t1390;
    let t26505 = t1983 * t26504;
    let t26533 = 0.135e2_f64 * t16521 * t1873;
    let t26535 = 27.0_f64 * t16524 * t7015;
    let t26537 = 0.135e2_f64 * t5371 * t6534;
    (t26502, t26503, t26504, t26505, t26533, t26535, t26537)
}
