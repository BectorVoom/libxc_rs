//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 810/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk810<F: Float>(t4827: F, t4839: F, t4842: F, t4845: F, t5008: F, t5020: F, t7015: F, t7020: F, t7021: F, t7025: F, t7031: F, t7033: F, t7036: F, t7052: F, t7055: F, t7093: F) -> F {
    let t7154 = -t5008 - t7015 - t4827 + t4839 - t7020 + t7021 - t5020 + t4842 + t7025 + t7031 + t7033 - t7036 - t4845 - t7052 + t7055 + t7093;
    t7154
}
