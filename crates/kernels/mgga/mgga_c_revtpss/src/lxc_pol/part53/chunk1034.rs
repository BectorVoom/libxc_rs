//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1034/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1034<F: Float>(t3140: F, t860: F, t8477: F, t31798: F, t25386: F, t31837: F, t31830: F, t644: F, t8621: F, t8622: F, t1925: F, t36: F) -> (F, F, F, F, F, F, F) {
    let t32425 = t860 * t3140;
    let t32426 = t8477 * t32425;
    let t32463 = t8477 * t31798;
    let t32469 = t25386 * t31837;
    let t32474 = t31830 * t31837;
    let t32586 = t8621 * t8622 * t644;
    let t32591 = t1925 * t36;
    (t32425, t32426, t32463, t32469, t32474, t32586, t32591)
}
