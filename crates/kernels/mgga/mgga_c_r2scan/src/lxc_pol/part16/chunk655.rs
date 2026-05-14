//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 655/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk655<F: Float>(t5314: F, t586: F, t4849: F, t4850: F, t4851: F, t4852: F, t4853: F, t5309: F, t5312: F) -> (F, F) {
    let t5315 = t586 * t5314;
    let t5317 = -0.17261666666666666667e1 * t5309 + 0.11507777777777777778e1 * t5312 - 0.53702962962962962964e1 * t5315 - t4849 + t4850 - t4851 - t4852 - t4853;
    (t5315, t5317)
}
