//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 564/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk564<F: Float>(t62: F, t694: F, t199: F, t2971: F, t119: F, t789: F, t203: F, t3743: F, t734: F) -> (F, F, F, F, F) {
    let t3745 = t62 * t694;
    let t3750 = t199 * t2971;
    let t3753 = t789 * t119;
    let t3758 = t203 * t3743;
    let t3767 = t734 * t3743;
    (t3745, t3750, t3753, t3758, t3767)
}
