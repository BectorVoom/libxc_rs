//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 632/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk632<F: Float>(t23076: F, t23081: F, t25999: F, t26004: F, t26009: F, t26014: F, t26019: F, t26022: F, t26025: F, t26029: F, t26033: F, t26036: F) -> F {
    let t26111 = t25999 + t26004 + t26009 / F::cast_from(4.0_f64) + t26014 / F::cast_from(4.0_f64) + t26019 / F::cast_from(4.0_f64) - t26022 / F::cast_from(3.0_f64) - t26025 / F::cast_from(12.0_f64) - t26029 - t26033 / F::cast_from(2.0_f64) + t26036 / F::cast_from(6.0_f64) - t23076 + t23081 / F::cast_from(3.0_f64);
    t26111
}
