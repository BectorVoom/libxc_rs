//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1005/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1005<F: Float>(t419: F, t420: F, t423: F, t85501: F, t45662: F, t58730: F, t85463: F, t85467: F, t85472: F, t85476: F, t85481: F, t85485: F, t85489: F, t85493: F, t85498: F) -> (F, F) {
    let t85504 = t419 * t420 * t423 * t85501;
    let t85506 = F::new(0.25537443351851851852e-1) * t85463 + F::new(0.34049924469135802469e-1) * t85467 - F::new(0.15322466011111111111e0) * t85472 + F::new(0.22983699016666666666e0) * t85476 + F::new(0.17024962234567901234e-1) * t58730 - F::new(0.23834947128395061728e0) * t85481 + F::new(0.11917473564197530864e0) * t85485 + F::new(0.30644932022222222222e0) * t85489 - F::new(0.30644932022222222222e0) * t85493 + F::new(0.94583123525377229081e-2) * t45662 + F::new(0.66208186467764060357e-1) * t85498 + F::new(0.6384360837962962963e-2) * t85504;
    (t85504, t85506)
}
