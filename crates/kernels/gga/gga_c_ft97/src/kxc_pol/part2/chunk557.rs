//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 557/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk557<F: Float>(t1073: F, t358: F, t2266: F, t363: F, t2281: F, t637: F, t643: F, t1640: F, t2289: F, t3042: F, t3045: F, t3048: F, t3054: F, t3359: F, t3363: F, t383: F) -> (F, F, F, F, F) {
    let t3635 = t1073 * t358;
    let t3637 = t2266 * t3635 * t363;
    let t3640 = t2281 * t1073;
    let t3642 = t637 * t3640 * t643;
    let t3653 = -F::new(0.117377e0) * t3359 * t383 + F::new(0.234754e0) * t3363 + t2289 + F::cast_from(0.4814361111111111111e-1_f64) * t1640 + F::cast_from(0.4814361111111111111e-1_f64) * t3042 - F::cast_from(0.9628722222222222222e-1_f64) * t3045 + F::cast_from(0.28886166666666666666e0_f64) * t3048 - F::cast_from(0.28886166666666666666e0_f64) * t3054;
    (t3635, t3637, t3640, t3642, t3653)
}
