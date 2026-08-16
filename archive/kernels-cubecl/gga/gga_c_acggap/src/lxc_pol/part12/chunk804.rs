//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 804/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk804<F: Float>(t2031: F, t507: F, t2030: F, t2061: F, t2060: F, t2314: F, t7447: F, t527: F, t7685: F, t1426: F, t2085: F, t535: F) -> (F, F, F, F, F, F, F) {
    let t8823 = t507 * t2031;
    let t8824 = t2030 * t8823;
    let t8826 = t507 * t2061;
    let t8827 = t2060 * t8826;
    let t8829 = t7447 * t2314;
    let t8835 = t7685 * t527;
    let t8838 = t1426 * t535 * t2085;
    (t8823, t8824, t8826, t8827, t8829, t8835, t8838)
}
