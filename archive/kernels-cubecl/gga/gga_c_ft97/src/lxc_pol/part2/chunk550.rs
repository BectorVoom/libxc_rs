//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 550/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk550<F: Float>(t3526: F, t579: F, t91: F, t3318: F, t3335: F, t1960: F, t1963: F, t2124: F, t3321: F, t3325: F, t3328: F, t3332: F, t3340: F, t3345: F, t3411: F, t3493: F) -> (F, F) {
    let t3528 = t91 * t579 * t3526;
    let t3530 = t3318 / F::cast_from(27.0_f64);
    let t3535 = t3335 / F::cast_from(9.0_f64);
    let t3539 = -t3493 / F::cast_from(12.0_f64) + t3528 / F::cast_from(6.0_f64) + t2124 + t1960 + t1963 + t3530 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t3321 + t3325 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3328 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3332 + t3535 + t3340 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3345 - t3411 / F::cast_from(3.0_f64);
    (t3528, t3539)
}
