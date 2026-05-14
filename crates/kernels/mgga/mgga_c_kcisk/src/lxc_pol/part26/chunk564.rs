//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 564/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk564<F: Float>(t1341: F, t5601: F, t1340: F, t5600: F, t2270: F, sigma0: F) -> (F, F, F, F) {
    let t5602 = t1341 * t5601;
    let t5603 = t1340 * t5602;
    let t5604 = t5600 * t5603;
    let t5606 = t2270 * sigma0;
    (t5602, t5603, t5604, t5606)
}
