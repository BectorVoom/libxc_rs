//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1179/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1179<F: Float>(t26788: F, t7309: F, t1349: F, t138438: F, t138715: F, t1969: F, t23413: F, t26551: F, t26561: F, t26809: F, t26909: F, t28: F, t3052: F, t32696: F, t32709: F, t32711: F, t32722: F, t33002: F, t35022: F, t3588: F, t5772: F, t5773: F, t6580: F, t7313: F, t925: F, t9432: F) -> F {
    let t149398 = t7309 * t26788;
    let t149404 = F::new(2.0) * t5772 * t9432 * t5773 * t26909 - t26809 * t1969 * t32722 * t3052 / F::new(9.0) - t5772 * t1969 * t138438 * t925 / F::new(18.0) - t1349 * t28 * t32709 * t26551 / F::new(3.0) + t23413 * t35022 / F::new(9.0) - t1349 * t28 * t7313 * t3588 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t6580 * t33002 - t6580 * t32711 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t138715 + t149398 / F::new(9.0) + t6580 * t32696 / F::new(3.0) + t7309 * t26561 / F::new(6.0);
    t149404
}
