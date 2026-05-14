//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 903/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk903<F: Float>(t19622: F, t19636: F, t203: F, t9078: F, t19507: F, t4017: F, t681: F, t1266: F, t186: F, t1457: F, t561: F, t1180: F, t4978: F, t5462: F, t1403: F, t1672: F) -> (F, F, F, F, F, F, F) {
    let t19639 = t19636 * t203 * t19622 * t9078;
    let t19644 = t19507 * t681 * t19622 * t4017;
    let t19652 = t1266 * t186;
    let t19670 = t561 * t1457;
    let t19671 = t19670 * t1180;
    let t19677 = t5462 * t4978;
    let t19686 = t1672 * t1403;
    (t19639, t19644, t19652, t19670, t19671, t19677, t19686)
}
