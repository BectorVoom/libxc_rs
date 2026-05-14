//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1023/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1023<F: Float>(t1164: F, t5679: F, t12813: F, t5732: F, t3409: F, t5612: F, t513: F, t943: F, t157: F, t1165: F, t14373: F, t1532: F, t1748: F, t864: F, t1432: F, t15947: F, t3361: F) -> (F, F, F, F, F, F, F) {
    let t20417 = t1164 * t5679;
    let t20422 = t12813 * t5732;
    let t20430 = t3409 * t5612;
    let t20432 = t513 * t943;
    let t20433 = t20432 * t157;
    let t20441 = t14373 * t1165 * t1532 * t1748 * t864;
    let t20446 = t3361 * t1165 * t15947 * t1432;
    (t20417, t20422, t20430, t20432, t20433, t20441, t20446)
}
