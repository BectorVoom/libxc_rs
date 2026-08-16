//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 302/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk302(t1256: f64, t5: f64, t629: f64, t1246: f64, t1248: f64, t512: f64, t537: f64, t541: f64, t546: f64, t593: f64, t600: f64, t605: f64) -> (f64, f64, f64) {
    let t1260 = t5 * t1256;
    let t1261 = t629 * t1260;
    let t1264 = t512 + t537 - t541 - t546 + t1246 + t593 + t1248 - t600 - t605;
    (t1260, t1261, t1264)
}
