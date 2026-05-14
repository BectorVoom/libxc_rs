//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 920/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk920<F: Float>(t2040: F, t7334: F, t1459: F, t8611: F, t670: F, t8453: F, t572: F, t7002: F, t7330: F, t8614: F, t116: F, t8460: F, t1936: F, t648: F, t94: F, t3140: F, t860: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32362 = t2040 * t7334;
    let t32365 = 6.0 * t1459 * t8611;
    let t32366 = t670 * t8453;
    let t32368 = 6.0 * t572 * t32366;
    let t32369 = t7330 * t7002;
    let t32371 = 12.0 * t572 * t32369;
    let t32372 = t1459 * t8614;
    let t32373 = 3.0 * t32372;
    let t32374 = t116 * t8460;
    let t32375 = t32374 * t670;
    let t32376 = t572 * t32375;
    let t32377 = 6.0 * t32376;
    let t32392 = t648 * t1936;
    let t32394 = t94 * t7002;
    let t32425 = t860 * t3140;
    (t32362, t32365, t32366, t32368, t32369, t32371, t32373, t32374, t32375, t32377, t32392, t32394, t32425)
}
