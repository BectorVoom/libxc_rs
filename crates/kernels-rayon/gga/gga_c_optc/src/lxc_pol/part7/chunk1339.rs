//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1339/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1339(t1081: f64, t8749: f64, t3029: f64, t3058: f64, t8697: f64, t26224: f64, t406: f64, t26214: f64, t1066: f64, t8882: f64, t2927: f64, t2973: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26722 = t1081 * t8749;
    let t26732 = t3029 * t3058;
    let t26735 = t1081 * t8697;
    let t26738 = t406 * t26224;
    let t26745 = t406 * t26214;
    let t26749 = t8882 * t1066;
    let t26754 = t2927 * t2973;
    (t26722, t26732, t26735, t26738, t26745, t26749, t26754)
}
