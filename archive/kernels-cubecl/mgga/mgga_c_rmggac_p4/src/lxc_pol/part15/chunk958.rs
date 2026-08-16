//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 958/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk958<F: Float>(t10088: F, t495: F, t511: F, t7230: F, t7231: F, t1737: F, t3351: F, t498: F, t880: F, t3352: F, t6394: F, t1971: F, t3924: F, t6397: F) -> (F, F, F, F) {
    let t45896 = t7230 * t7231 * t511 * t10088 * t495;
    let t45901 = t3351 * t7231 * t880 * t1737 * t498;
    let t45905 = t3351 * t3352 * t880 * t6394;
    let t45909 = t3351 * t1971 * t3924 * t6397;
    (t45896, t45901, t45905, t45909)
}
