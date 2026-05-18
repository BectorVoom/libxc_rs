//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 695/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk695<F: Float>(t23657: F, t27078: F, t590: F, t920: F, t23671: F, t5916: F, t1359: F, t3526: F, t586: F, t28: F, t5890: F, t1039: F, t5842: F) -> (F, F, F, F, F) {
    let t27079 = t23657 * t27078;
    let t27081 = t920 * t590;
    let t27083 = t23671 * t5916 * t27081;
    let t27084 = t23657 * t27083;
    let t27086 = t1359 * t3526;
    let t27087 = t586 * t27086;
    let t27089 = t5890 * t28 * t27087;
    let t27091 = t5842 * t1039;
    (t27079, t27081, t27084, t27089, t27091)
}
