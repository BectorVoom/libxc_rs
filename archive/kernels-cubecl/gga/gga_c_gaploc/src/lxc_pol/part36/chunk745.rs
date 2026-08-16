//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 745/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk745<F: Float>(t2558: F, t28438: F, t10036: F, t1980: F, t10928: F, t6574: F, t822: F, t123: F, t15499: F, t27997: F, t7290: F, t28013: F) -> (F, F, F, F, F, F) {
    let t28439 = t28438 * t2558;
    let t28594 = t1980 * t10036;
    let t28640 = t822 * t10928 * t6574;
    let t28641 = t15499 * t123;
    let t28648 = t7290 * t27997;
    let t28652 = t7290 * t28013;
    (t28439, t28594, t28640, t28641, t28648, t28652)
}
