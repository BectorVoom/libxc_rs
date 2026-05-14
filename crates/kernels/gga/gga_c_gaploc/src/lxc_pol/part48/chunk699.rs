//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 699/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk699<F: Float>(t26984: F, t9294: F, t12953: F, t31054: F, t12986: F, t2464: F, t2487: F, t3177: F, t35091: F, t9272: F, t12987: F, t7014: F, t2365: F, t31558: F, t7025: F, t12943: F, t4379: F) -> (F, F, F, F, F, F, F) {
    let t42189 = t26984 * t9294;
    let t42199 = t31054 * t12953;
    let t42202 = t2487 * t2464 * t12986;
    let t42226 = t9272 * t35091 * t3177;
    let t42256 = t7014 * t12987;
    let t42259 = t7025 * t2365 * t31558;
    let t42316 = t4379 * t12943;
    (t42189, t42199, t42202, t42226, t42256, t42259, t42316)
}
