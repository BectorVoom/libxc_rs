//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1076/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1076<F: Float>(t10701: F, t1841: F, t10632: F, t5524: F, t2925: F, t935: F) -> (F, F, F) {
    let t32352 = t1841 * t10701;
    let t32353 = 0.85450291446024714264e-3 * t32352;
    let t32355 = 0.25635087433807414278e-2 * t5524 * t10632;
    let t32356 = t2925 * t935;
    (t32353, t32355, t32356)
}
