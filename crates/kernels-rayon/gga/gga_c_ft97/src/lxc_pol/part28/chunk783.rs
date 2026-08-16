//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 783/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk783(t32493: f64, t32539: f64, t32590: f64, t32639: f64, t1286: f64, t32392: f64, t32396: f64, t32401: f64, t32403: f64, t32406: f64, t32412: f64, t32415: f64, t32417: f64, t32420: f64, t32425: f64, t32428: f64, t32458: f64, t32470: f64, t32474: f64, t32546: f64, t32550: f64, t438: f64, t5501: f64, t5510: f64, t7162: f64, t7286: f64, t88: f64) -> (f64, f64) {
    let t32641 = t32493 + t32539 + t32590 + t32639;
    let t32649 = -t1286 * t32392 / 3.0_f64 + t1286 * t32396 / 3.0_f64 + t32401 + t1286 * t32403 - 2.0_f64 / 3.0_f64 * t1286 * t32406 - t7162 * t5510 / 3.0_f64 + 8.0_f64 * t32412 + 4.0_f64 * t32415 + 8.0_f64 * t32417 - 12.0_f64 * t32420 - t438 * t7286 - t5501 * t32425 / 9.0_f64 - t88 * t32641 - 2.0_f64 * t32458 - 2.0_f64 * t32470 + 4.0_f64 * t32474 - 4.0_f64 * t32428 - 2.0_f64 * t32546 - 4.0_f64 * t32550;
    (t32641, t32649)
}
