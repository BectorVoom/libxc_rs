//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 591/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk591<F: Float>(t10425: F, t3407: F, t7014: F, t123: F, t2754: F, t883: F, t2488: F, t2487: F, t2465: F, t2787: F, t2464: F, t2778: F) -> (F, F, F, F, F, F) {
    let t10426 = F::cast_from(0.14896037479937677779e-1_f64) * t10425;
    let t10427 = t7014 * t3407;
    let t10428 = F::cast_from(0.19171462976960374838e0_f64) * t10427;
    let t10429 = t2754 * t123;
    let t10430 = t10429 * t883;
    let t10431 = t2488 * t10430;
    let t10432 = t2487 * t10431;
    let t10433 = F::cast_from(0.19171462976960374838e0_f64) * t10432;
    let t10434 = t2465 * t2787;
    let t10435 = t2464 * t10434;
    let t10436 = t2487 * t10435;
    let t10437 = F::cast_from(0.42603251059911944084e-1_f64) * t10436;
    let t10438 = t2465 * t2778;
    (t10426, t10428, t10430, t10433, t10437, t10438)
}
