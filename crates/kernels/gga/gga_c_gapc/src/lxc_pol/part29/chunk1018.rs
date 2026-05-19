//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1018/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1018<F: Float>(t4978: F, t5462: F, t1403: F, t1672: F, t3005: F, t5216: F, t1784: F, t1908: F, t1911: F, t1845: F, t186: F, t1803: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t19677 = t5462 * t4978;
    let t19686 = t1672 * t1403;
    let t19765 = t3005 * t5216;
    let t19771 = pi * t1784 * t1908 * t1911;
    let t19844 = t1845 * t186;
    let t19902 = t1803 * t186;
    (t19677, t19686, t19765, t19771, t19844, t19902)
}
