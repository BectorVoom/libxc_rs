//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1123/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1123<F: Float>(t10301: F, t34409: F, t2247: F, t29362: F, t8435: F, t60224: F, t8736: F, t10309: F, t13272: F, t32801: F, t122885: F, t45972: F, t125312: F, t2121: F, t136: F, t29411: F) -> (F, F, F, F, F, F, F, F) {
    let t129165 = t10301 * t34409;
    let t129169 = t2247 * t8435 * t29362;
    let t129180 = t60224 * t8736;
    let t129193 = t10309 * t34409;
    let t129213 = t13272 * t32801;
    let t129216 = t45972 * t122885;
    let t129232 = t2247 * t125312 * t2121;
    let t129236 = t2247 * t29411 * t136;
    (t129165, t129169, t129180, t129193, t129213, t129216, t129232, t129236)
}
