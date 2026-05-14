//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 896/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk896<F: Float>(t12975: F, t19100: F, t25590: F, t25601: F, t25609: F, t30569: F, t30572: F, t30592: F, t30595: F, t30599: F, t30603: F, t1180: F, t1173: F, t5690: F, t7764: F, t25696: F, t25699: F, t25701: F, t30582: F, t30585: F) -> (F, F, F, F, F) {
    let t30605 = -t12975 - 4.0 / 9.0 * t19100 + 2.0 / 9.0 * t25590 - 2.0 / 3.0 * t25601 + t25609 / 3.0 - 10.0 / 27.0 * t30592 + 4.0 / 3.0 * t30595 - 2.0 / 3.0 * t30569 - 2.0 * t30599 + 2.0 * t30572 - t30603 / 3.0;
    let t30606 = t1180 * t30605;
    let t30608 = t1173 * t30605;
    let t30610 = t5690 * t7764;
    let t30612 = -0.59793333333333333333e0 * t30569 + 0.17938e1 * t30572 - 0.39862222222222222223e0 * t19100 + 0.19931111111111111111e0 * t25590 - 0.59793333333333333333e0 * t25601 + 0.29896666666666666667e0 * t25609 - 0.32862666666666666666e0 * t25696 + 0.16431333333333333333e0 * t25699 + 0.5477111111111111111e-1 * t25701 - 0.82156666666666666668e-1 * t30582 + 0.49293999999999999999e0 * t30585 + 0.3071625e0 * t30606 + 0.1898925e1 * t30608 - 0.28483875e1 * t30610;
    (t30605, t30606, t30608, t30610, t30612)
}
