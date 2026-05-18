//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 691/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk691<F: Float>(t13397: F, t2488: F, t2487: F, t123: F, t3529: F, t883: F, t912: F, t587: F, t13261: F, t1457: F, t1572: F, t11318: F, t874: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13398 = t2488 * t13397;
    let t13399 = t2487 * t13398;
    let t13401 = t3529 * t123;
    let t13402 = t13401 * t883;
    let t13403 = t912 * t13402;
    let t13404 = t587 * t13403;
    let t13405 = F::new(0.19171462976960374838e0) * t13404;
    let t13415 = t1457 * t13261;
    let t13417 = F::new(0.71500979903700853338e0) * t1572 * t13415;
    let t13420 = t11318 * t874;
    (t13398, t13399, t13401, t13402, t13403, t13405, t13415, t13417, t13420)
}
