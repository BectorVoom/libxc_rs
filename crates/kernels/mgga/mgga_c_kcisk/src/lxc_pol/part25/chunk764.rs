//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 764/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk764<F: Float>(t2049: F, t2815: F, t9702: F, t9706: F, t9710: F, t9712: F, t9714: F, t9716: F) -> (F, F) {
    let t9763 = t2815 * t2049;
    let t9772 = 0.9375e-1 * t9702 - 0.9375e-1 * t9706 + 0.625e-1 * t9710 - 0.20234375e-1 * t9712 + 0.20234375e-1 * t9714 - 0.26979166666666666667e-1 * t9716;
    (t9763, t9772)
}
