//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1091/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1091<F: Float>(t1967: F, t9681: F, t1988: F, t9531: F, t429: F, t4352: F, t598: F, t9529: F, t368: F, t5655: F, t1980: F, t30058: F) -> (F, F, F, F, F) {
    let t39112 = t1967 * t9681;
    let t39114 = t1988 * t9531;
    let t39118 = t598 * t4352 * t429 * t9529;
    let t39120 = t368 * t5655;
    let t39122 = t1980 * t30058 * t39120;
    (t39112, t39114, t39118, t39120, t39122)
}
