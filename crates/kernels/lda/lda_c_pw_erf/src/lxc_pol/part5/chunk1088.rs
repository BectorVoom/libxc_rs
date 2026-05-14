//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1088/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1088<F: Float>(t20823: F, t3974: F, t5160: F, t22595: F, t3976: F, t549: F, t4506: F, t593: F, t6728: F, t22596: F, t4515: F, t22584: F, t22586: F, t22587: F, t22588: F, t22589: F, t22590: F, t22591: F, t22594: F, t22599: F) -> (F, F, F, F, F) {
    let t22602 = 16.0 / 15.0 * t3974 * t5160 * t20823;
    let t22606 = 8.0 / 15.0 * t3974 * t3976 * t22595 * t549;
    let t22610 = 8.0 / 15.0 * t4506 * t6728 * t22595 * t593;
    let t22613 = 8.0 / 15.0 * t4506 * t4515 * t22596;
    let t22614 = t22584 + t22586 - t22587 - t22588 - t22589 + t22590 - t22591 - t22594 - t22599 - t22602 - t22606 + t22610 + t22613;
    (t22602, t22606, t22610, t22613, t22614)
}
