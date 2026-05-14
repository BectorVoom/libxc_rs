//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 674/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk674<F: Float>(t2130: F, t4567: F, t493: F, t1518: F, t812: F, t548: F, t219: F, t573: F, t558: F, t2070: F, t211: F, t1524: F, t835: F, t2001: F, t3854: F, t1318: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4568 = t4567 * t2130;
    let t4569 = t493 * t4568;
    let t4570 = 4.0 / 9.0 * t4569;
    let t4571 = t1518 * t812;
    let t4572 = t548 * t4571;
    let t4573 = 8.0 / 135.0 * t4572;
    let t4574 = t573 * t219;
    let t4575 = t4574 * t558;
    let t4576 = t2070 * t4575;
    let t4578 = 8.0 / 45.0 * t211 * t4576;
    let t4580 = 4.0 / 15.0 * t1524 * t835;
    let t4581 = t3854 * t2001;
    let t4583 = 32.0 / 135.0 * t1318 * t4581;
    (t4568, t4570, t4571, t4573, t4574, t4575, t4576, t4578, t4580, t4581, t4583)
}
