//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1400/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1400<F: Float>(t26261: F, t26264: F, t26252: F, t26258: F, t26326: F, t26328: F, t26330: F, t26332: F, t26351: F, t26354: F, t26358: F, t26314: F, t26339: F, t26343: F, t26363: F, t26365: F, t26367: F, t26369: F, t26372: F, t26376: F, t26379: F, t26382: F, t26385: F) -> (F, F) {
    let t27866 = F::cast_from(0.20106419753086419753e2_f64) * t26261;
    let t27867 = F::cast_from(0.20068888888888888889e-1_f64) * t26264;
    let t27875 = F::cast_from(0.28723456790123456789e1_f64) * t26252 + F::cast_from(0.2585111111111111111e2_f64) * t26258 + t27866 + t27867 - F::cast_from(0.51702222222222222221e1_f64) * t26326 - F::cast_from(0.34468148148148148146e1_f64) * t26328 - F::cast_from(0.6568e-2_f64) * t26351 + F::cast_from(0.6568e-2_f64) * t26354 + F::cast_from(0.10340444444444444444e2_f64) * t26330 + F::cast_from(0.8042567901234567901e1_f64) * t26332 + F::cast_from(0.14595555555555555556e-1_f64) * t26358;
    let t27888 = -F::cast_from(0.57446913580246913579e1_f64) * t26339 - F::cast_from(0.19388333333333333333e1_f64) * t26343 - F::cast_from(0.821e-2_f64) * t26363 - F::cast_from(0.27366666666666666666e-2_f64) * t26365 + F::cast_from(0.3284e-2_f64) * t26367 + F::cast_from(0.14595555555555555556e-2_f64) * t26369 + F::cast_from(0.25851111111111111111e1_f64) * t26314 + F::cast_from(0.1642e-1_f64) * t26372 - F::cast_from(0.12771111111111111111e-2_f64) * t26376 - F::cast_from(0.12315e-2_f64) * t26379 - F::cast_from(0.3284e-2_f64) * t26382 + F::cast_from(0.9852e-2_f64) * t26385;
    (t27875, t27888)
}
