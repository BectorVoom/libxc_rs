//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 363/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk363<F: Float>(t1666: F, t567: F, t1665: F, t144: F, t672: F, t203: F, t674: F) -> (F, F, F, F) {
    let t1667 = t1666 * t567;
    let t1668 = t1665 * t1667;
    let t1671 = t672 * t144;
    let t1672 = t674 * t203;
    (t1667, t1668, t1671, t1672)
}
