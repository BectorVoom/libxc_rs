//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 945/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk945(t13934: f64, t731: f64, t13937: f64, t2549: f64, t12176: f64, t2558: f64, t943: f64, t1843: f64, t39149: f64, t7064: f64, t2562: f64, t38974: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47652 = t731 * t13934;
    let t47687 = t2549 * t13937;
    let t47690 = t943 * t12176 * t2558;
    let t47702 = t731 * t13937;
    let t47731 = t7064 * t1843 * t39149;
    let t47768 = t2549 * t13934;
    let t47772 = t943 * t2562 * t883 * t38974;
    (t47652, t47687, t47690, t47702, t47731, t47768, t47772)
}
