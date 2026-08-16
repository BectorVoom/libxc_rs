//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 924/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk924(t2: f64, t5053: f64, t2372: f64, t713: f64, t4934: f64, t9707: f64, t3821: f64, t3930: f64, t13306: f64, t13308: f64, t13329: f64, t13335: f64, t13338: f64, t13339: f64, t13345: f64, t13388: f64, t13680: f64, t13682: f64, t13688: f64, t18271: f64, t18276: f64, t18279: f64, t18283: f64, t18286: f64, t462: f64, t9907: f64, t9935: f64, t9936: f64) -> f64 {
    let t18288 = t2 * t5053;
    let t18290 = t2372 * t18288 * t713;
    let t18293 = t2 * t4934;
    let t18295 = t9707 * t18293 * t713;
    let t18299 = t2372 * t3930 * t3821;
    let t18302 = -t13306 + t13308 - t13329 - 8.0_f64 / 27.0_f64 * t13335 - t13338 - 4.0_f64 / 9.0_f64 * t13339 - 4.0_f64 / 9.0_f64 * t9936 + t13345 + 4.0_f64 / 9.0_f64 * t13682 * t18271 - 4.0_f64 / 3.0_f64 * t13688 * t18276 - 4.0_f64 / 3.0_f64 * t13688 * t18279 - 4.0_f64 / 27.0_f64 * t9907 - 2.0_f64 / 9.0_f64 * t18283 - t9935 - t13388 - 8.0_f64 / 9.0_f64 * t13680 + t18286 / 9.0_f64 + 2.0_f64 * t462 * t18290 - 6.0_f64 * t462 * t18295 + 4.0_f64 * t462 * t18299;
    t18302
}
