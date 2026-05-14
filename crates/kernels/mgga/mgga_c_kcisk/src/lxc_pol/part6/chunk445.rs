//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 445/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk445<F: Float>(t3783: F, t453: F, t1413: F, t394: F, sigma0: F) -> (F, F, F, F) {
    let t3784 = t453 * t3783;
    let t3785 = t3784 * sigma0;
    let t3795 = t1413 * sigma0;
    let t3796 = t3795 * t394;
    (t3784, t3785, t3795, t3796)
}
