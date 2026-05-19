//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1050/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1050<F: Float>(t86: F, t112: F, t113: F, t20479: F, t20489: F, t4628: F, t4635: F, t5: F, t86541: F, t86559: F, t86571: F, t989: F, t992: F) -> F {
    let t87 = F::cast_from(10000000.0_f64) <= t86;
    let t86576 = piecewise3::<F>(t87, F::new(0.0), t5 * (t86541 + t86559) * t113 / F::new(4.0) + t5 * t20479 * t992 + F::new(3.0) / F::new(2.0) * t5 * t4628 * t4635 + t5 * t989 * t20489 + t5 * t112 * t86571 / F::new(4.0));
    t86576
}
