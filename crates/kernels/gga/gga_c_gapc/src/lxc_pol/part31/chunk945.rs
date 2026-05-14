//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 945/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk945<F: Float>(t12633: F, t12637: F, t12644: F, t12648: F, t12042: F, t12048: F, t12049: F, t12051: F, t12057: F, t12060: F, t12287: F, t12290: F, t12293: F, t12586: F, t12588: F, t12589: F, t12623: F, t2464: F, t3914: F, t884: F) -> (F, F) {
    let t12650 = t12633 + t12637 + t12644 + t12648;
    let t12653 = -t12650 * t884 - t2464 * t3914 + t12042 - t12048 + t12049 - t12051 - t12057 - t12060 + t12287 - t12290 + t12293 - t12586 + t12588 + t12589 + t12623;
    (t12650, t12653)
}
