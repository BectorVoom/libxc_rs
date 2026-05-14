//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 953/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk953<F: Float>(t12637: F, t3788: F, t4738: F, t5068: F, t518: F, t2168: F, t10508: F, t826: F, t558: F, t581: F, t2151: F, t571: F, t593: F, t12622: F, t12624: F, t12626: F, t12628: F, t12630: F, t12632: F, t12634: F, t12636: F) -> (F, F, F, F, F, F, F) {
    let t12638 = 8.0 / 135.0 * t12637;
    let t12639 = t4738 * t3788;
    let t12640 = 16.0 / 15.0 * t12639;
    let t12641 = t5068 * t518;
    let t12643 = 8.0 / 5.0 * t12641 * t2168;
    let t12645 = 8.0 / 15.0 * t10508 * t826;
    let t12646 = t581 * t558;
    let t12650 = 8.0 / 15.0 * t571 * t2151 * t12646 * t593;
    let t12651 = -t12622 - t12624 - t12626 - t12628 + t12630 + t12632 - t12634 + t12636 - t12638 - t12640 - t12643 + t12645 + t12650;
    (t12638, t12640, t12641, t12643, t12645, t12650, t12651)
}
