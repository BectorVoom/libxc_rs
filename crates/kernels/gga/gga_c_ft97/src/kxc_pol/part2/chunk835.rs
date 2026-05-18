//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 835/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk835<F: Float>(t12362: F, t12365: F, t12353: F, t12359: F, t12564: F, t12568: F, t13117: F, t9166: F, t9372: F, t9373: F, t9380: F, t12571: F) -> (F, F) {
    let t13119 = F::new(4.0) / F::new(27.0) * t12362;
    let t13120 = F::new(2.0) / F::new(3.0) * t12365;
    let t13122 = t9372 + t9373 - t9380 + F::new(4.0) * t12353 - t13117 + F::new(22.0) / F::new(9.0) * t12359 - t13119 - t9166 + t13120 - t12564 - F::new(2.0) / F::new(3.0) * t12568;
    let t13123 = F::new(4.0) / F::new(9.0) * t12571;
    (t13122, t13123)
}
