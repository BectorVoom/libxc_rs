//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1088/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1088<F: Float>(t1457: F, t561: F, t1180: F, t4978: F, t5462: F, t1403: F, t1672: F, t3005: F, t5216: F, t1784: F, t1908: F, t1911: F) -> (F, F, F, F, F, F) {
    let t19670 = t561 * t1457;
    let t19671 = t19670 * t1180;
    let t19677 = t5462 * t4978;
    let t19686 = t1672 * t1403;
    let t19765 = t3005 * t5216;
    let t19771 = M_PI * t1784 * t1908 * t1911;
    (t19670, t19671, t19677, t19686, t19765, t19771)
}
