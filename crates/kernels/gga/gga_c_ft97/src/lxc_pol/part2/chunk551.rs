//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 551/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk551<F: Float>(t143: F, t160: F, t3539: F, t1030: F, t1882: F, t167: F, t3408: F, t574: F, t1055: F, t1959: F, t1962: F, t2149: F, t3318: F, t3321: F, t3325: F, t3328: F, t3332: F, t3335: F, t3340: F, t3345: F, t3411: F, t3493: F, t3528: F) -> (F, F, F, F, F) {
    let t3541 = t143 * t3539 * t160;
    let t3545 = t1882 * t1030;
    let t3548 = t574 * t167 * t3408;
    let t3551 = t1882 * t1055;
    let t3565 = -t3493 / F::cast_from(4.0_f64) + t3528 / F::cast_from(2.0_f64) + t2149 + t1959 / F::cast_from(9.0_f64) + t1962 / F::cast_from(3.0_f64) + t3318 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3321 + t3325 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3328 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3332 + t3335 / F::cast_from(3.0_f64) + t3340 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t3345 - t3411;
    (t3541, t3545, t3548, t3551, t3565)
}
