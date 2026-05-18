//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 823/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk823<F: Float>(t2464: F, t2465: F, t825: F, t9595: F, t12669: F, t2013: F, t9953: F, t7427: F, t9734: F, t12660: F, t2902: F, t9243: F) -> (F, F, F, F, F, F) {
    let t41532 = t825 * t2464 * t2465 * t9595;
    let t41534 = t2013 * t12669;
    let t41538 = t825 * t2464 * t2465 * t9953;
    let t41542 = t7427 * t2464 * t2465 * t9734;
    let t41544 = t2013 * t12660;
    let t41574 = t9243 * t2902;
    (t41532, t41534, t41538, t41542, t41544, t41574)
}
