//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 775/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk775(t432: f64, t7274: f64, t1852: f64, t452: f64, t32420: f64, t83: f64, t5644: f64, t5710: f64, t1901: f64, t32496: f64, t32500: f64, t32504: f64, t32508: f64, t32510: f64, t32512: f64, t32517: f64, t32520: f64, t32524: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t32527 = t7274 * t432;
    let t32529 = t452 * t1852 * t32527;
    let t32532 = t83 * t32420;
    let t32536 = t452 * t5710 * t5644;
    let t32539 = -2.0_f64 / 9.0_f64 * t1901 * t32496 - 2.0_f64 / 3.0_f64 * t446 * t32500 + 2.0_f64 / 3.0_f64 * t446 * t32504 - t32508 + t32510 - 2.0_f64 / 3.0_f64 * t446 * t32512 + t1901 * t32517 / 9.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t32520 + 2.0_f64 / 3.0_f64 * t446 * t32524 - 2.0_f64 / 3.0_f64 * t446 * t32529 - 2.0_f64 * t446 * t32532 + 2.0_f64 / 3.0_f64 * t446 * t32536;
    (t32527, t32529, t32532, t32536, t32539)
}
