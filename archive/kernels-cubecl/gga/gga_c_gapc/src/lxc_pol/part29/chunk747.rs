//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 747/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk747<F: Float>(t1689: F, t2997: F, t1038: F, t1431: F, t2996: F, t128: F, t644: F, t640: F) -> (F, F, F, F) {
    let t8763 = t2997 * t1689;
    let t8764 = t1038 * t1431;
    let t8765 = t8763 * t8764;
    let t8766 = t2996 * t8765;
    let t8768 = t128 * t644;
    let t8769 = t640 * t8768;
    (t8765, t8766, t8768, t8769)
}
