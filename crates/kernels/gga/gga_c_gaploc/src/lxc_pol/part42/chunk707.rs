//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 707/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk707<F: Float>(t123: F, t3689: F, t883: F, t912: F, t587: F, t2488: F, t2487: F, t12079: F, t901: F, t2366: F, t2365: F, t1429: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13777 = t3689 * t123;
    let t13778 = t13777 * t883;
    let t13779 = t912 * t13778;
    let t13780 = t587 * t13779;
    let t13782 = t2488 * t13778;
    let t13783 = t2487 * t13782;
    let t13789 = t12079 * t901;
    let t13791 = t2366 * t3689;
    let t13792 = t2365 * t13791;
    let t13793 = t1429 * t13792;
    (t13778, t13779, t13780, t13782, t13783, t13789, t13791, t13792, t13793)
}
