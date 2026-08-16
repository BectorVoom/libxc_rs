//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 948/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk948(t11981: f64, t2464: f64, t2465: f64, t2487: f64, t13782: f64, t7014: f64, t13791: f64, t1429: f64, t549: f64, t13779: f64, t1407: f64, t38674: f64, t544: f64) -> (f64, f64, f64, f64, f64) {
    let t47883 = t2487 * t2464 * t2465 * t11981;
    let t47885 = t7014 * t13782;
    let t47892 = t1429 * t549 * t13791;
    let t47949 = t1407 * t13779;
    let t47964 = t544 * t38674;
    (t47883, t47885, t47892, t47949, t47964)
}
