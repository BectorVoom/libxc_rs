//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1437/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1437<F: Float>(t13141: F, t1770: F, t13126: F, t1209: F, t21455: F, t5219: F, t5477: F, t5462: F, t21451: F, t17191: F, t3566: F, t13147: F) -> (F, F, F, F, F, F, F, F) {
    let t59498 = t1770 * t13141;
    let t59550 = t1770 * t13126;
    let t59674 = t1209 * t21455;
    let t59681 = t5219 * t5477;
    let t59749 = t5219 * t5462;
    let t59788 = t1209 * t21451;
    let t59817 = t3566 * t17191;
    let t59948 = t1770 * t13147;
    (t59498, t59550, t59674, t59681, t59749, t59788, t59817, t59948)
}
