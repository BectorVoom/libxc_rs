//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 798/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk798<F: Float>(t45043: F, t7474: F, t1970: F, t1971: F, t236: F, t6178: F, t1704: F, t209: F, t476: F, t9188: F, t1707: F, t3352: F, t495: F, t511: F, t7230: F, t40658: F, t9222: F) -> (F, F, F, F, F) {
    let t45044 = t7474 * t45043;
    let t45048 = t1970 * t1971 * t236 * t6178;
    let t45055 = t1970 * t9188 * t236 * t1704 * t476 * t209;
    let t45060 = t7230 * t3352 * t511 * t1707 * t495;
    let t45062 = t9222 * t40658;
    (t45044, t45048, t45055, t45060, t45062)
}
