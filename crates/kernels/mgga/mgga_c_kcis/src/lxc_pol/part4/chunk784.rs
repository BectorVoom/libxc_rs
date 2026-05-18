//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 784/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk784<F: Float>(t3211: F, t4805: F, t3210: F, t3200: F, t1133: F, t1773: F) -> (F, F, F, F) {
    let t4806 = t3211 * t4805;
    let t4807 = t3210 * t4806;
    let t4808 = t3200 * t4807;
    let t4813 = t1773 * t1133;
    (t4806, t4807, t4808, t4813)
}
