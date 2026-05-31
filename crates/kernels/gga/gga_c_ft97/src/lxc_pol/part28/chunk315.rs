//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 315/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk315<F: Float>(t1901: F, t2164: F, t2195: F, t3281: F, t3421: F, t3426: F, t3431: F, t3436: F, t3442: F, t3447: F, t3452: F, t3457: F, t3460: F, t3463: F, t3467: F, t3471: F, t446: F) -> F {
    let t3474 = t2195 / F::cast_from(27.0_f64) + t1901 * t3421 / F::cast_from(9.0_f64) + t1901 * t3426 / F::cast_from(9.0_f64) + t1901 * t3431 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t3436 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t3442 + t1901 * t3447 / F::cast_from(9.0_f64) + t2164 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t3452 + t446 * t3457 / F::cast_from(3.0_f64) + t3460 / F::cast_from(27.0_f64) - t446 * t3463 / F::cast_from(9.0_f64) - t446 * t3467 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3281 * t3471;
    t3474
}
