//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 553/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk553<F: Float>(t4623: F, t6: F, t2024: F, t161: F, t1256: F, t3360: F, t2034: F) -> (F, F, F, F, F) {
    let t4624 = t6 * t4623;
    let t4625 = t4624 * t2024;
    let t4626 = t161 * t4625;
    let t4630 = t3360 * t1256;
    let t4631 = t2034 * t4630;
    (t4624, t4625, t4626, t4630, t4631)
}
