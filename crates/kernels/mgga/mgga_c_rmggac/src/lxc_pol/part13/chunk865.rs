//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 865/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk865<F: Float>(t36772: F, t9147: F, t1971: F, t615: F, t7230: F, t839: F, t880: F, t1587: F, t236: F, t3351: F, t498: F, t7248: F, t26157: F, t5223: F, t645: F, t1635: F, t2064: F, t4044: F) -> (F, F, F, F, F) {
    let t41696 = t36772 * t9147;
    let t41701 = t7230 * t1971 * t880 * t615 * t839;
    let t41706 = t3351 * t7248 * t236 * t1587 * t498;
    let t41713 = t26157 * t645 * t5223;
    let t41716 = t4044 * t2064 * t1635;
    (t41696, t41701, t41706, t41713, t41716)
}
