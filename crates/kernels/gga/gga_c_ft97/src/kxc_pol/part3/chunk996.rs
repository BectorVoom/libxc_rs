//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 996/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk996<F: Float>(t15168: F, t15170: F, t19375: F, t19380: F, t19384: F, t19387: F, t19389: F, t19392: F, t19396: F, t19401: F, t19406: F, t19411: F, t19415: F, t19420: F, t19425: F, t446: F) -> F {
    let t19428 = t446 * t19375 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t19380 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t19384 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t19387 - t15168 - t15170 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19389 - t446 * t19392 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t19396 - F::cast_from(2.0_f64) * t446 * t19401 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t19406 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t19411 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t19415 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t19420 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t19425;
    t19428
}
