//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 953/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk953(t1063: f64, t38267: f64, t894: f64, t13725: f64, t484: f64, t197: f64, t3689: f64, t161: f64, t1358: f64, t2339: f64, t13735: f64, t6305: f64) -> (f64, f64, f64, f64, f64) {
    let t47001 = t1063 * t894 * t38267;
    let t47003 = t484 * t13725;
    let t47008 = t197 * t3689;
    let t47009 = t47008 * t161;
    let t47011 = t1358 * t47009 * t2339;
    let t47013 = t6305 * t13735;
    (t47001, t47003, t47008, t47011, t47013)
}
