//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1002/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1002<F: Float>(t100: F, t369: F, t110: F, t38482: F, t488: F, t8275: F, t3170: F, t463: F, t3056: F, t71: F, t526: F, t597: F, t2178: F, t3539: F, t167: F, t40436: F) -> (F, F, F, F, F, F, F, F) {
    let t47667 = t369 * t100;
    let t47768 = t38482 * t110;
    let t47799 = t8275 * t488;
    let t47831 = t463 * t3170;
    let t49004 = t71 * t3056;
    let t49414 = t526 * t597;
    let t49562 = t3539 * t2178;
    let t49579 = t40436 * t167;
    (t47667, t47768, t47799, t47831, t49004, t49414, t49562, t49579)
}
