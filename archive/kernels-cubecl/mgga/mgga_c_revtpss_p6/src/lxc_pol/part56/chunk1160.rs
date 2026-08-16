//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1160/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1160<F: Float>(t122820: F, t28067: F, t196: F, t197: F, t29437: F, t2035: F, t34399: F, t7313: F, t28166: F, t8763: F, t28168: F, t28043: F, t7586: F) -> (F, F, F, F, F) {
    let t129366 = t122820 * t28067;
    let t129370 = t29437 * t196 * t197;
    let t129371 = t129370 * t2035;
    let t129376 = t34399 * t7313;
    let t129377 = t8763 * t28166;
    let t129378 = t129377 * t28168;
    let t129395 = t7586 * t28043;
    (t129366, t129371, t129376, t129378, t129395)
}
