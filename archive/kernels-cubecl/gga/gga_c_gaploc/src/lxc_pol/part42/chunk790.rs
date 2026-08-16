//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 790/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk790<F: Float>(t12990: F, t30733: F, t27003: F, t587: F, t9438: F, t2487: F, t27007: F, t26328: F, t6914: F, t2321: F, t35215: F, t9074: F) -> (F, F, F, F, F) {
    let t42412 = t12990 * t30733;
    let t42420 = t587 * t9438 * t27003;
    let t42428 = t2487 * t9438 * t27007;
    let t42431 = t6914 * t9438 * t26328;
    let t42539 = t9074 * t35215 * t2321;
    (t42412, t42420, t42428, t42431, t42539)
}
