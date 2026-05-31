//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 776/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk776<F: Float>(t11003: F, t3621: F, t3618: F, t8675: F, t3622: F, t1068: F, t8640: F, t171: F, t7741: F, t11: F, t41: F, t3630: F) -> (F, F, F, F, F, F) {
    let t12158 = t3621 * t11003;
    let t12162 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t8675 * t3618;
    let t12164 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t8675 * t3622;
    let t12165 = t8640 * t1068;
    let t12168 = F::cast_from(1.0_f64) / t171 / t7741;
    let t12169 = t11 * t12168;
    let t12170 = t41 * t12169;
    let t12171 = t12170 * t3630;
    (t12158, t12162, t12164, t12165, t12170, t12171)
}
