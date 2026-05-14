//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 508/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk508<F: Float>(t495: F, t498: F, t236: F, t7231: F, t7230: F, t321: F, t3352: F, t464: F, t483: F, t1968: F, t1966: F) -> (F, F, F, F, F, F) {
    let t7232 = t495 * t498;
    let t7233 = t236 * t7232;
    let t7234 = t7231 * t7233;
    let t7235 = t7230 * t7234;
    let t7236 = 0.1064114997332445985e-4 * t7235;
    let t7237 = t495 * t321;
    let t7238 = t236 * t7237;
    let t7239 = t3352 * t7238;
    let t7240 = t7230 * t7239;
    let t7241 = 0.31923449919973379548e-4 * t7240;
    let t7242 = t464 * t483;
    let t7243 = t7242 * t1968;
    let t7244 = t1966 * t7243;
    (t7234, t7236, t7239, t7241, t7243, t7244)
}
