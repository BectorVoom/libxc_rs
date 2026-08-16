//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 504/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk504<F: Float>(t14091: F, t7557: F, t3069: F, t7494: F, t2186: F, t3154: F, t2044: F, t7554: F, t2139: F, t3157: F, t3807: F, t2048: F, t236: F) -> (F, F, F, F, F, F, F) {
    let t14092 = t14091 * t7557;
    let t14094 = t7494 * t3069;
    let t14100 = t2186 * t3154;
    let t14102 = t2044 * t7554;
    let t14103 = t2139 * t14102;
    let t14105 = t3807 * t3157;
    let t14107 = t236 * t2048;
    (t14092, t14094, t14100, t14102, t14103, t14105, t14107)
}
