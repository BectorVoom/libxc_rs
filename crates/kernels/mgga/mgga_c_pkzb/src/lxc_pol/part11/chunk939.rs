//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 939/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk939<F: Float>(t24: F, t11139: F, t3152: F, t3820: F, t898: F, t10523: F, t6097: F, t10528: F, t821: F, t3019: F, t3374: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t11140 = 0.1434375e0 * t11139;
    let t11141 = t3152 * t3820;
    let t11143 = 0.35089341735807877242e1 * t898 * t11141;
    let t11146 = t6097 * t10523;
    let t11150 = t821 * t10528;
    let t11153 = piecewise3(t90, 0.0, -28.0 / 27.0 * t11146 + 4.0 / 3.0 * t3019 * t3374 - t11150 / 3.0);
    (t11140, t11141, t11143, t11146, t11150, t11153)
}
