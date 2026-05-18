//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 507/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk507<F: Float>(t4162: F, t4165: F, t4160: F, t552: F, t491: F, sigma2: F) -> (F, F, F, F) {
    let t4166 = t4162 * t4165;
    let t4167 = t4160 * t4166;
    let t4169 = t552 * sigma2;
    let t4170 = t4169 * t491;
    (t4166, t4167, t4169, t4170)
}
