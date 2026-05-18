//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1178/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1178<F: Float>(t15204: F, t1570: F, t17536: F, t17539: F, t17542: F, t23: F, t429: F, t17500: F, t3058: F, t17650: F, t4215: F, t17618: F, t4296: F) -> (F, F, F, F, F) {
    let t53327 = t1570 * t15204;
    let t53332 = t17536 * t17539 * t23 * t429 * t17542;
    let t53361 = t3058 * t17500;
    let t53367 = t17650 * t4215;
    let t53390 = t17618 * t4296;
    (t53327, t53332, t53361, t53367, t53390)
}
