//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 859/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk859<F: Float>(t15168: F, t15170: F, t19375: F, t19380: F, t19384: F, t19387: F, t19389: F, t19392: F, t19396: F, t19401: F, t19406: F, t19411: F, t19415: F, t19420: F, t19425: F, t446: F) -> (F,) {
    let t19428 = t446 * t19375 / 3.0 + 2.0 / 3.0 * t446 * t19380 + 4.0 / 3.0 * t446 * t19384 - 2.0 / 27.0 * t19387 - t15168 - t15170 + 2.0 / 9.0 * t19389 - t446 * t19392 / 3.0 - 2.0 / 3.0 * t446 * t19396 - 2.0 * t446 * t19401 - 2.0 / 3.0 * t446 * t19406 + 4.0 / 3.0 * t446 * t19411 + 2.0 / 3.0 * t446 * t19415 + 2.0 / 3.0 * t446 * t19420 - 2.0 / 3.0 * t446 * t19425;
    (t19428,)
}
