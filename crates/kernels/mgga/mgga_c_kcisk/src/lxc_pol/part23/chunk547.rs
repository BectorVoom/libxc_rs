//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 547/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk547<F: Float>(t3776: F, t3777: F, t1340: F, t1411: F, t1412: F, t466: F) -> (F, F, F, F) {
    let t3778 = t3776 * t3777;
    let t3779 = t1340 * t3778;
    let t3780 = t1411 * t3779;
    let t3783 = 1.0 / t1412 / t466;
    (t3778, t3779, t3780, t3783)
}
