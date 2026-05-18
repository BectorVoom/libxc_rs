//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 859/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk859<F: Float>(t3593: F, t4599: F, t1256: F, t13509: F, t13526: F, t16287: F, t16337: F, t16339: F, t16340: F, t1879: F, t3539: F, t4595: F, t606: F, t6477: F, t6811: F, t95: F, t9535: F) -> F {
    let t16595 = t3593 * t4599;
    let t16602 = t16337 + F::new(3.0) / F::new(2.0) * t9535 + t16339 + F::new(0.23260393291413087447e-1) * t1879 * t3593 * t4595 + F::new(0.77534644304710291488e-2) * t95 * t606 * t16287 + F::new(0.46520786582826174894e-1) * t3539 * t16595 + t6811 + F::new(3.0) / F::new(2.0) * t13526 + t6477 - F::new(0.23260393291413087447e-1) * t1879 * t13509 * t1256 + t16340;
    t16602
}
