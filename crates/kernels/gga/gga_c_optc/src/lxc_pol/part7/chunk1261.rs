//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1261/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1261<F: Float>(t26261: F, t26264: F, t26252: F, t26258: F, t26326: F, t26328: F, t26330: F, t26332: F, t26351: F, t26354: F, t26358: F, t26314: F, t26339: F, t26343: F, t26363: F, t26365: F, t26367: F, t26369: F, t26372: F, t26376: F, t26379: F, t26382: F, t26385: F) -> (F, F) {
    let t27866 = 0.20106419753086419753e2 * t26261;
    let t27867 = 0.20068888888888888889e-1 * t26264;
    let t27875 = 0.28723456790123456789e1 * t26252 + 0.2585111111111111111e2 * t26258 + t27866 + t27867 - 0.51702222222222222221e1 * t26326 - 0.34468148148148148146e1 * t26328 - 0.6568e-2 * t26351 + 0.6568e-2 * t26354 + 0.10340444444444444444e2 * t26330 + 0.8042567901234567901e1 * t26332 + 0.14595555555555555556e-1 * t26358;
    let t27888 = -0.57446913580246913579e1 * t26339 - 0.19388333333333333333e1 * t26343 - 0.821e-2 * t26363 - 0.27366666666666666666e-2 * t26365 + 0.3284e-2 * t26367 + 0.14595555555555555556e-2 * t26369 + 0.25851111111111111111e1 * t26314 + 0.1642e-1 * t26372 - 0.12771111111111111111e-2 * t26376 - 0.12315e-2 * t26379 - 0.3284e-2 * t26382 + 0.9852e-2 * t26385;
    (t27875, t27888)
}
