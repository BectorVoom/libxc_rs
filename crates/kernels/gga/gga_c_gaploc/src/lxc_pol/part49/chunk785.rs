//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 785/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk785<F: Float>(t41416: F, t825: F, t969: F, t2464: F, t2465: F, t9595: F, t12669: F, t2013: F, t9953: F, t7427: F, t9734: F, t12660: F, t10305: F, t6556: F, t2902: F, t9243: F) -> (F, F, F, F, F, F, F, F) {
    let t41528 = t825 * t969 * t41416;
    let t41532 = t825 * t2464 * t2465 * t9595;
    let t41534 = t2013 * t12669;
    let t41538 = t825 * t2464 * t2465 * t9953;
    let t41542 = t7427 * t2464 * t2465 * t9734;
    let t41544 = t2013 * t12660;
    let t41572 = t6556 * t10305;
    let t41574 = t9243 * t2902;
    (t41528, t41532, t41534, t41538, t41542, t41544, t41572, t41574)
}
