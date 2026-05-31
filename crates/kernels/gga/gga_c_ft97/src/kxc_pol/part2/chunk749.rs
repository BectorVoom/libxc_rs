//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 749/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk749<F: Float>(t11715: F, t11771: F, t457: F, t91: F, t11069: F, t11041: F, t11048: F, t11052: F, t11056: F, t11061: F, t11066: F, t11073: F, t11659: F, t7771: F) -> (F, F) {
    let t11772 = t11715 + t11771;
    let t11774 = t91 * t457 * t11772;
    let t11778 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11069;
    let t11780 = -F::cast_from(2.0_f64) * t11041 - t11659 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11048 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11052 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11056 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11061 + t11774 / F::cast_from(6.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t7771 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11066 + t11778 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11073;
    (t11774, t11780)
}
