//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 538/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk538<F: Float>(t10425: F, t3407: F, t7014: F, t123: F, t2754: F, t883: F) -> (F, F, F, F) {
    let t10426 = F::new(0.14896037479937677779e-1) * t10425;
    let t10427 = t7014 * t3407;
    let t10428 = F::new(0.19171462976960374838e0) * t10427;
    let t10429 = t2754 * t123;
    let t10430 = t10429 * t883;
    (t10426, t10427, t10428, t10430)
}
