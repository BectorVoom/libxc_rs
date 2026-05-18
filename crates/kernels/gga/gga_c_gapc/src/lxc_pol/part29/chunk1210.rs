//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1210/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1210<F: Float>(t1030: F, t33748: F, t8853: F, t11601: F, t9278: F, t26698: F, t33399: F, t8362: F, t8784: F, t11302: F, t19902: F, t20596: F) -> (F, F, F, F) {
    let t34940 = t1030 * t33748 * t8853;
    let t34942 = t11601 * t9278;
    let t34946 = t8784 * t33399 * t8362 * t26698;
    let t34949 = t19902 * t11302 * t20596;
    (t34940, t34942, t34946, t34949)
}
