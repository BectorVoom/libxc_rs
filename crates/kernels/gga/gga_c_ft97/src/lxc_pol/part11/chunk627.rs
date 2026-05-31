//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 627/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk627<F: Float>(t1643: F, t643: F, t8654: F, t2265: F, t631: F, t8621: F, t8626: F, t8630: F, t8636: F, t8641: F, t8643: F, t8645: F, t8647: F, t8650: F, t8652: F) -> (F, F) {
    let t8655 = t1643 * t643;
    let t8656 = t8654 * t8655;
    let t8659 = F::cast_from(6.0_f64) * t631 * t8621 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t631 * t8626 + t631 * t8630 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t631 * t8636 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t8641 - t8643 / F::cast_from(3.0_f64) - t8645 / F::cast_from(9.0_f64) + F::cast_from(3.0_f64) * t8647 + t631 * t8650 - t2265 * t8652 - t2265 * t8656 / F::cast_from(3.0_f64);
    (t8656, t8659)
}
