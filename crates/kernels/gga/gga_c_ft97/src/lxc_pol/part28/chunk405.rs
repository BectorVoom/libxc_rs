//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 405/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk405<F: Float>(t5674: F, t6501: F, t1800: F, t6469: F, t1317: F, t28: F, t469: F, t6454: F, t5691: F, t920: F, t1564: F, t446: F, t5507: F, t942: F, t89: F, t370: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6502 = t5674 * t6501;
    let t6504 = t1800 * t6469;
    let t6506 = t1317 * t28 * t6504;
    let t6508 = t469 * t6454;
    let t6510 = t1317 * t28 * t6508;
    let t6512 = t5691 * t920;
    let t6513 = t1564 * t6512;
    let t6514 = t446 * t6513;
    let t6516 = t5507 * t942;
    let t6517 = t28 * t6516;
    let t6518 = t89 * t6517;
    let t6520 = t370 * t6454;
    (t6502, t6504, t6506, t6508, t6510, t6512, t6513, t6514, t6516, t6517, t6518, t6520)
}
