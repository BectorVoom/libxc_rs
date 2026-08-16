//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 919/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk919<F: Float>(t1131: F, t2569: F, t2568: F, t729: F, t1882: F, t3848: F, t1170: F, t8232: F, t3953: F, t681: F, t89: F, t2469: F, t3859: F) -> (F, F, F, F, F) {
    let t14226 = t1131 * t2569;
    let t14228 = t729 * t2568 * t14226;
    let t14232 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1882 * t3848;
    let t14233 = t8232 * t1170;
    let t14240 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t89 * t681 * t3953;
    let t14242 = t729 * t2469 * t3859;
    (t14228, t14232, t14233, t14240, t14242)
}
