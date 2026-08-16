//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 937/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk937(t13725: f64, t484: f64, t197: f64, t3689: f64, t161: f64, t1365: f64, t38272: f64, t6525: f64, t13740: f64, t13847: f64, t825: f64, t826: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47003 = t484 * t13725;
    let t47008 = t197 * t3689;
    let t47009 = t47008 * t161;
    let t47036 = t6525 * t1365 * t38272;
    let t47042 = t484 * t13740;
    let t47140 = t825 * t826 * t13847;
    (t47003, t47008, t47009, t47036, t47042, t47140)
}
