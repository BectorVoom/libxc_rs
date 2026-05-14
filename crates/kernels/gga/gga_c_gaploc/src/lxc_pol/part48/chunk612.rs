//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 612/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk612<F: Float>(t12993: F, t2487: F, t10268: F, t2365: F, t4391: F, t3005: F, t3295: F, t9800: F, t11053: F, t9805: F, t1029: F, t9796: F, t123: F, t3431: F, t883: F, t969: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12994 = t2487 * t12993;
    let t12996 = t2365 * t10268;
    let t12997 = t4391 * t12996;
    let t13052 = t3005 * t3295;
    let t13053 = t9800 * t13052;
    let t13055 = t11053 * t3295;
    let t13056 = t9805 * t13055;
    let t13058 = t1029 * t3295;
    let t13059 = t9796 * t13058;
    let t13063 = t3431 * t123;
    let t13064 = t13063 * t883;
    let t13065 = t969 * t13064;
    (t12994, t12996, t12997, t13052, t13053, t13055, t13056, t13058, t13059, t13064, t13065)
}
