//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 650/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk650(t26356: f64, t379: f64, t1902: f64, t6466: f64, t8392: f64, t1901: f64, t26319: f64, t26322: f64, t26326: f64, t26330: f64, t26334: f64, t26337: f64, t26340: f64, t26343: f64, t26346: f64, t26350: f64, t26353: f64, t3281: f64, t446: f64) -> (f64, f64) {
    let t26357 = t26356 * t379;
    let t26358 = t1902 * t26357;
    let t26361 = t8392 * t6466;
    let t26363 = -t1901 * t26319 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t26322 + 2.0_f64 / 3.0_f64 * t446 * t26326 - 2.0_f64 / 9.0_f64 * t3281 * t26330 - t446 * t26334 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t26337 - t446 * t26340 / 3.0_f64 + t1901 * t26343 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t26346 - 2.0_f64 / 27.0_f64 * t1901 * t26350 + t1901 * t26353 / 9.0_f64 + t1901 * t26358 / 9.0_f64 - t26361 / 27.0_f64;
    (t26357, t26363)
}
