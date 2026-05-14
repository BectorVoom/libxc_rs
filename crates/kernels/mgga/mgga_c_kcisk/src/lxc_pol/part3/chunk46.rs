//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 46/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk46<F: Float>(t12: F, t15: F) -> (F, F, F, F) {
    let t146 = 0.107924e1 + 0.3964e-1 * t15 + 0.123825e-1 * t12;
    let t149 = 1.0 + t15 * t146 / 2.0;
    let t150 = t149 * t149;
    let t151 = 1.0 / t150;
    (t146, t149, t150, t151)
}
