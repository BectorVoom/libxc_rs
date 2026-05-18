//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 485/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk485<F: Float>(t1341: F, t3786: F, t3785: F, t1411: F, t1440: F, t3764: F, t1415: F, t1413: F, t394: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t3787 = t1341 * t3786;
    let t3788 = t3785 * t3787;
    let t3789 = t1411 * t3788;
    let t3791 = t3764 * t1440;
    let t3792 = t1415 * t3791;
    let t3793 = t1411 * t3792;
    let t3795 = t1413 * sigma0;
    let t3796 = t3795 * t394;
    (t3787, t3788, t3789, t3791, t3792, t3793, t3795, t3796)
}
