//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1041/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1041<F: Float>(t352: F, t8492: F, t481: F, t986: F, t795: F, t113: F, t5086: F, t1065: F, t1563: F, t104: F, t494: F, t1543: F) -> (F, F, F, F, F, F) {
    let t31929 = t352 * t8492;
    let t32094 = t986 * t481;
    let t32212 = t986 * t795;
    let t36967 = t113 * t5086;
    let t36968 = t1065 * t1563;
    let t36969 = t36967 * t36968;
    let t36985 = t104 * t494;
    let t36987 = t1065 * t1543;
    (t31929, t32094, t32212, t36969, t36985, t36987)
}
