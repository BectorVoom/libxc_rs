//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 791/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk791<F: Float>(t10256: F, t30204: F, t6525: F, t10272: F, t2317: F, t12830: F, t1358: F, t3129: F, t31903: F, t9074: F, t10166: F, t9086: F) -> (F, F, F, F, F) {
    let t42546 = t6525 * t30204 * t10256;
    let t42579 = t6525 * t10272 * t2317;
    let t42581 = t1358 * t12830;
    let t42587 = t9074 * t31903 * t3129;
    let t42590 = t9074 * t10166 * t9086;
    (t42546, t42579, t42581, t42587, t42590)
}
