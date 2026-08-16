//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 792/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk792<F: Float>(t2487: F, t2488: F, t40190: F, t2365: F, t29985: F, t4391: F, t1429: F, t30140: F, t29854: F, t29970: F, t6963: F, t12526: F, t587: F, t589: F) -> (F, F, F, F, F, F) {
    let t40546 = t2487 * t2488 * t40190;
    let t40549 = t4391 * t2365 * t29985;
    let t40555 = t1429 * t2365 * t30140;
    let t40558 = t4391 * t2365 * t29854;
    let t40561 = t6963 * t2365 * t29970;
    let t40564 = t587 * t589 * t12526;
    (t40546, t40549, t40555, t40558, t40561, t40564)
}
