//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 815/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk815<F: Float>(t41572: F, t2902: F, t9243: F, t2798: F, t9588: F, t10624: F, t2355: F, t10295: F, t19933: F, t24215: F, t3366: F, t13001: F, t1377: F) -> (F, F, F, F, F, F, F) {
    let t41573 = F::new(4.0) * t41572;
    let t41574 = t9243 * t2902;
    let t41575 = t2798 * t9588;
    let t41576 = t2355 * t10624;
    let t41577 = F::new(2.0) * t41576;
    let t41579 = F::new(12.0) * t19933 * t10295;
    let t41581 = F::new(4.0) * t24215 * t3366;
    let t41582 = t1377 * t13001;
    (t41573, t41574, t41575, t41577, t41579, t41581, t41582)
}
