//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 628/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk628<F: Float>(t211: F, t4561: F, t1518: F, t785: F, t493: F, t1: F, t1124: F, t2130: F, t812: F, t548: F, t219: F, t573: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4562 = t211 * t4561;
    let t4564 = t1518 * t785;
    let t4565 = t493 * t4564;
    let t4567 = t1 * t1124;
    let t4568 = t4567 * t2130;
    let t4569 = t493 * t4568;
    let t4571 = t1518 * t812;
    let t4572 = t548 * t4571;
    let t4574 = t573 * t219;
    (t4562, t4564, t4565, t4567, t4568, t4569, t4571, t4572, t4574)
}
