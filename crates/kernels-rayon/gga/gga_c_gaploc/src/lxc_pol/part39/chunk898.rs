//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 898/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk898(t2464: f64, t2465: f64, t825: f64, t9595: f64, t12669: f64, t2013: f64, t9953: f64, t7427: f64, t9734: f64, t12660: f64, t10305: f64, t6556: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41532 = t825 * t2464 * t2465 * t9595;
    let t41534 = t2013 * t12669;
    let t41538 = t825 * t2464 * t2465 * t9953;
    let t41542 = t7427 * t2464 * t2465 * t9734;
    let t41544 = t2013 * t12660;
    let t41572 = t6556 * t10305;
    (t41532, t41534, t41538, t41542, t41544, t41572)
}
