//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 790/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk790<F: Float>(t2084: F, t27: F, t7282: F, t794: F, t2160: F, t638: F, t7224: F, t2184: F, t465: F, t7472: F, t7478: F, t118: F, t1995: F, t2001: F, t498: F) -> (F, F, F, F, F, F) {
    let t36715 = t7282 * t27 * t2084 * t794;
    let t36718 = t638 * t2160 * t7224;
    let t36733 = t465 * t2184;
    let t36734 = t7472 * t36733;
    let t36735 = t36734 * t7478;
    let t36740 = t2001 * t118 * t1995 * t498;
    (t36715, t36718, t36733, t36734, t36735, t36740)
}
