//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2006/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2006<F: Float>(t839: F, t93048: F, t25260: F, t820: F, t843: F, t2726: F, t10841: F, t25245: F, t10867: F, t64: F, t2681: F, t7043: F) -> (F, F, F, F, F) {
    let t93049 = t93048 * t839;
    let t93054 = t820 * t25260 * t843;
    let t93055 = t93054 * t2726;
    let t93058 = t25245 * t10841;
    let t93060 = t10867 * t64;
    let t93066 = t820 * t7043 * t2681;
    (t93049, t93055, t93058, t93060, t93066)
}
