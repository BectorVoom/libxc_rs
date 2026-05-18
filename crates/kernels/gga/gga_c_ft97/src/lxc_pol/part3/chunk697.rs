//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 697/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk697<F: Float>(t2253: F, t3642: F, t1736: F, t179: F, t3627: F, t41: F, t70: F, t3618: F, t8675: F, t3622: F, t1068: F, t8640: F) -> (F, F, F, F, F, F) {
    let t12132 = F::new(2.0) * t2253 * t3642;
    let t12137 = t1736 * t179;
    let t12143 = t41 * t3627 * t70;
    let t12162 = F::new(4.0) / F::new(9.0) * t8675 * t3618;
    let t12164 = F::new(4.0) / F::new(9.0) * t8675 * t3622;
    let t12165 = t8640 * t1068;
    (t12132, t12137, t12143, t12162, t12164, t12165)
}
