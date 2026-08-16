//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 984/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk984(t13238: f64, t5559: f64, t841: f64, t34013: f64, t977: f64, t3073: f64, t9767: f64, t5552: f64, t10687: f64, t2554: f64, t7064: f64, t13200: f64, t29439: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42912 = t5559 * t13238 * t841;
    let t42914 = t34013 * t977;
    let t42916 = t9767 * t3073;
    let t42917 = t5552 * t13238;
    let t42931 = t7064 * t10687 * t2554;
    let t42933 = t29439 * t13200;
    (t42912, t42914, t42916, t42917, t42931, t42933)
}
