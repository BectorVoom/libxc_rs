//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1249/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1249<F: Float>(t8229: F, t921: F, t7895: F, t947: F, t7373: F, t7433: F, t8127: F, t8129: F, t2367: F, t7920: F, t930: F, t2670: F, t288: F) -> (F, F, F, F, F, F) {
    let t25791 = t921 * t8229;
    let t25793 = t947 * t7895;
    let t25797 = t7433 * t7373;
    let t25799 = t8127 * t25797 * t8129;
    let t25804 = t930 * t2367 * t7920;
    let t25806 = t288 * t2670;
    (t25791, t25793, t25797, t25799, t25804, t25806)
}
