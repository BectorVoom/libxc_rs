//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 886/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk886<F: Float>(t10058: F, t3325: F, t134: F, t2404: F, t3412: F, t3405: F, t3411: F, t2315: F, t2801: F, t6: F, t3414: F, t9722: F) -> (F, F, F, F, F, F) {
    let t10059 = t3325 * t10058;
    let t10061 = t134 * t2404;
    let t10062 = t3412 * t10061;
    let t10063 = t3405 * t10062;
    let t10064 = t3411 * t10063;
    let t10067 = t134 * t2315;
    let t10068 = t2801 * t6 * t10067;
    let t10069 = t3405 * t10068;
    let t10070 = t3411 * t10069;
    let t10072 = t9722 * t3414;
    (t10059, t10063, t10064, t10069, t10070, t10072)
}
