//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1144/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1144<F: Float>(t17276: F, t973: F, t16857: F, t2418: F, t16890: F, t2367: F, t999: F, t17314: F, t176: F, t998: F, t16918: F, t4038: F, t8152: F) -> (F, F, F, F, F) {
    let t49417 = t17276 * t973;
    let t49581 = t16857 * t2418;
    let t49707 = t999 * t2367 * t16890;
    let t49754 = t176 * t17314 * t998;
    let t49773 = t4038 * t8152 * t16918;
    (t49417, t49581, t49707, t49754, t49773)
}
