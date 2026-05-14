//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 819/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk819<F: Float>(t25135: F, t852: F, t1486: F, t193: F, t1476: F, t2360: F, t2349: F, t2665: F, t446: F, t6327: F, t681: F, t1934: F, t6334: F, t1491: F, t1636: F, t89: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25136 = t852 * t25135;
    let t25138 = t1486 * t193 * t25136;
    let t25140 = t1476 * t2360;
    let t25141 = t25140 * t2349;
    let t25142 = t2665 * t25141;
    let t25143 = t446 * t25142;
    let t25146 = t1486 * t681 * t6327;
    let t25148 = t6334 * t1934;
    let t25149 = t2665 * t25148;
    let t25150 = t446 * t25149;
    let t25153 = t89 * t1636 * t1491;
    (t25136, t25138, t25140, t25142, t25143, t25146, t25149, t25150, t25153)
}
