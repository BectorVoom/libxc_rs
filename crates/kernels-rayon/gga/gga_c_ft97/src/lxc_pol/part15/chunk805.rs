//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 805/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk805(t21522: f64, t21551: f64, t21696: f64, t21768: f64, t21717: f64, t258: f64, t1137: f64, t1173: f64, t21123: f64, t21125: f64, t21464: f64, t21500: f64, t21532: f64, t21540: f64, t21548: f64, t21640: f64, t21688: f64, t247: f64, t263: f64, t4915: f64, t5059: f64, t5179: f64) -> (f64, f64, f64) {
    let t21770 = t21522 + t21551 + t21696 + t21768;
    let t21772 = t21717 * t258;
    let t21780 = -3.0_f64 * t1137 * t5179 - 3.0_f64 * t1173 * t4915 - 3.0_f64 * t1173 * t5059 - t21123 * t263 - 2.0_f64 * t21125 * t263 - t21464 * t263 - t21770 * t247 + 12.0_f64 * t21500 - 12.0_f64 * t21532 - 6.0_f64 * t21540 - 6.0_f64 * t21548 - 2.0_f64 * t21640 + 12.0_f64 * t21688 + 2.0_f64 * t21772;
    (t21770, t21772, t21780)
}
