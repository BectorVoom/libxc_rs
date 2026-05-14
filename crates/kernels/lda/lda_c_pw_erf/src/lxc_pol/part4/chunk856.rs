//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 856/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk856<F: Float>(t6580: F, t786: F, t2468: F, t565: F, t2114: F, t2505: F, t1298: F, t2849: F, t462: F, t198: F, t186: F, t493: F, t6475: F, t6477: F, t6481: F, t6485: F, t6487: F, t6491: F, t6495: F, t6570: F, t6572: F, t6574: F, t6576: F, t6578: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6582 = 8.0 / 15.0 * t6580 * t786;
    let t6584 = 4.0 / 15.0 * t565 * t2468;
    let t6586 = 4.0 / 15.0 * t2114 * t2505;
    let t6588 = 4.0 / 15.0 * t1298 * t2505;
    let t6590 = -t462 - 3.0 * t2849;
    let t6591 = t198 * t6590;
    let t6592 = t186 * t6591;
    let t6594 = 4.0 / 15.0 * t493 * t6592;
    let t6595 = t6475 + t6477 - t6481 - t6485 - t6487 + t6491 + t6495 - t6570 + t6572 - t6574 - t6576 - t6578 + t6582 + t6584 + t6586 + t6588 + t6594;
    (t6582, t6584, t6586, t6588, t6590, t6591, t6592, t6594, t6595)
}
