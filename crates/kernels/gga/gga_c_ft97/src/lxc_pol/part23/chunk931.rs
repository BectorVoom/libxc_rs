//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 931/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk931<F: Float>(t6308: F, t681: F, t7063: F, t7021: F, t824: F, t2781: F, t1486: F, t193: F, t1212: F, t6260: F, t1476: F, t4129: F, t856: F, t852: F, t4255: F, t6334: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28494 = t6308 * t681 * t7063;
    let t28496 = t7021 * t824;
    let t28497 = t2781 * t28496;
    let t28499 = t1486 * t193 * t28497;
    let t28501 = t6260 * t1212;
    let t28502 = t2781 * t28501;
    let t28504 = t1486 * t193 * t28502;
    let t28506 = t1476 * t4129;
    let t28507 = t2781 * t28506;
    let t28509 = t1486 * t193 * t28507;
    let t28511 = t7021 * t856;
    let t28512 = t852 * t28511;
    let t28514 = t6308 * t193 * t28512;
    let t28516 = t6334 * t4255;
    (t28494, t28496, t28497, t28499, t28501, t28502, t28504, t28506, t28507, t28509, t28512, t28514, t28516)
}
