//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 893/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk893<F: Float>(t1016: F, t1382: F, t9588: F, t2902: F, t3207: F, t12862: F, t4342: F, t12859: F, t4349: F, t605: F, t10301: F, t6556: F) -> (F, F, F, F, F) {
    let t42491 = F::cast_from(2.0_f64) * t1382 * t1016 * t9588;
    let t42494 = F::cast_from(2.0_f64) * t1382 * t2902 * t3207;
    let t42496 = F::cast_from(2.0_f64) * t4342 * t12862;
    let t42498 = t4349 * t12859 * t605;
    let t42499 = F::cast_from(12.0_f64) * t42498;
    let t42501 = F::cast_from(4.0_f64) * t6556 * t10301;
    (t42491, t42494, t42496, t42499, t42501)
}
