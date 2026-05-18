//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1221/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1221<F: Float>(t8763: F, t8995: F, t28199: F, t2163: F, t28042: F, t651: F, t122820: F, t28067: F, t196: F, t197: F, t29437: F, t2035: F) -> (F, F, F, F) {
    let t129353 = t8763 * t8995;
    let t129354 = t129353 * t28199;
    let t129357 = t651 * t2163 * t28042;
    let t129366 = t122820 * t28067;
    let t129370 = t29437 * t196 * t197;
    let t129371 = t129370 * t2035;
    (t129354, t129357, t129366, t129371)
}
