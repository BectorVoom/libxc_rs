//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 439/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk439<F: Float>(t27: F, t6681: F, t89: F, t5898: F, t5915: F, t6659: F, t6663: F, t6667: F, t6671: F, t6675: F, t6679: F) -> (F, F) {
    let t6683 = t89 * t27 * t6681;
    let t6685 = t6659 / F::cast_from(12.0_f64) + t5898 + t6663 / F::cast_from(18.0_f64) + t6667 / F::cast_from(3.0_f64) - t6671 / F::cast_from(6.0_f64) + t5915 + t6675 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6679 - t6683 / F::cast_from(3.0_f64);
    (t6683, t6685)
}
