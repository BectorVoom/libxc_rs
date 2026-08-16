//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 890/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk890<F: Float>(t38273: F, t446: F, t7793: F, t1588: F, t1651: F, t7824: F, t1882: F, t7816: F, t1647: F, t1755: F, t1564: F, t1546: F, t7746: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t38275 = t446 * t7793 * t38273;
    let t38277 = t1651 * t1588;
    let t38279 = t446 * t7824 * t38277;
    let t38281 = t1882 * t7816;
    let t38283 = t1647 * t1755;
    let t38285 = t446 * t1564 * t38283;
    let t38288 = t89 * t1546 * t7746;
    (t38275, t38277, t38279, t38281, t38283, t38285, t38288)
}
