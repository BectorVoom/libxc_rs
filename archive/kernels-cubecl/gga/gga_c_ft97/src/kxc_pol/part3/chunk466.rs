//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 466/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk466<F: Float>(t1026: F, t1882: F, t1060: F, t379: F, t569: F, t616: F, t925: F, t167: F, t3052: F, t1901: F, t2164: F, t2195: F, t3281: F, t3421: F, t3426: F, t3431: F, t3436: F, t3442: F, t3447: F, t3452: F, t3457: F, t446: F) -> (F, F, F, F, F) {
    let t3460 = t1882 * t1026;
    let t3463 = t569 * t1060 * t379;
    let t3467 = t569 * t616 * t925;
    let t3471 = t569 * t167 * t3052;
    let t3474 = t2195 / F::cast_from(27.0_f64) + t1901 * t3421 / F::cast_from(9.0_f64) + t1901 * t3426 / F::cast_from(9.0_f64) + t1901 * t3431 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t3436 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t3442 + t1901 * t3447 / F::cast_from(9.0_f64) + t2164 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t3452 + t446 * t3457 / F::cast_from(3.0_f64) + t3460 / F::cast_from(27.0_f64) - t446 * t3463 / F::cast_from(9.0_f64) - t446 * t3467 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3281 * t3471;
    (t3460, t3463, t3467, t3471, t3474)
}
