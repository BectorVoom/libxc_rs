//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 704/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk704<F: Float>(t2487: F, t2488: F, t40190: F, t2365: F, t29985: F, t4391: F, t1429: F, t30140: F, t29854: F, t29970: F, t6963: F, t12526: F, t587: F, t589: F, t6985: F, t30209: F, t7025: F) -> (F, F, F, F, F, F, F, F) {
    let t40546 = t2487 * t2488 * t40190;
    let t40549 = t4391 * t2365 * t29985;
    let t40555 = t1429 * t2365 * t30140;
    let t40558 = t4391 * t2365 * t29854;
    let t40561 = t6963 * t2365 * t29970;
    let t40564 = t587 * t589 * t12526;
    let t40567 = t2487 * t6985 * t12526;
    let t40570 = t7025 * t2365 * t30209;
    (t40546, t40549, t40555, t40558, t40561, t40564, t40567, t40570)
}
