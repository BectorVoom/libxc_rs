//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 348/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk348<F: Float>(t1110: F, t1111: F, t1121: F, t1131: F, t1133: F, t1499: F, t1503: F, t1509: F, t1516: F, t1520: F, t431: F, t451: F) -> F {
    let t1523 = -t1499 * t431 / F::new(36.0) + t1110 + t1111 * t1503 / F::new(288.0) + F::new(0.35500316489081544176e-1) * t1121 * t1509 - F::new(0.14488602482981263091e-1) * t1516 * t451 + t1131 + F::new(0.18110753103726578864e-2) * t1133 * t1520;
    t1523
}
