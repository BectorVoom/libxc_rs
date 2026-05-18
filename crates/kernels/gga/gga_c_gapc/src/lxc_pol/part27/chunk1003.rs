//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1003/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1003<F: Float>(t15541: F, t2456: F, t3787: F, t7521: F, t6851: F, t871: F, t2440: F, t286: F, t5: F, t4: F, t8139: F, t8140: F, t941: F) -> (F, F, F, F, F, F, F) {
    let t15542 = t15541 * t2456;
    let t15548 = t3787 * t7521;
    let t15553 = t871 * t6851;
    let t15555 = t15541 * t2440;
    let t15608 = t5 * t286;
    let t15609 = t15608 * t4;
    let t15610 = t8139 * t15609;
    let t15615 = t941 * t8140;
    (t15542, t15548, t15553, t15555, t15609, t15610, t15615)
}
