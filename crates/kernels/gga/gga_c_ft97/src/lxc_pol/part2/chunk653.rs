//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 653/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk653<F: Float>(t3255: F, t432: F, t452: F, t488: F, t100: F, t370: F, t1851: F, t979: F, t1876: F, t3103: F, t492: F, t1820: F, t942: F, t10967: F, t83: F, t10962: F) -> (F, F, F, F, F, F) {
    let t11485 = t3255 * t432;
    let t11487 = t452 * t488 * t11485;
    let t11490 = t370 * t100;
    let t11491 = t1851 * t979;
    let t11492 = t11491 * t1876;
    let t11493 = t11490 * t11492;
    let t11496 = t3103 * t492;
    let t11498 = t452 * t488 * t11496;
    let t11501 = t942 * t1820;
    let t11503 = t452 * t488 * t11501;
    let t11506 = t83 * t10967;
    let t11509 = t83 * t10962;
    (t11487, t11493, t11498, t11503, t11506, t11509)
}
