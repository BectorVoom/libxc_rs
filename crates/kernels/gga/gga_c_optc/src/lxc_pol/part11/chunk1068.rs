//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1068/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1068<F: Float>(t11782: F, t17515: F, t4297: F, t18187: F, t4281: F, t9142: F, t15083: F, t15107: F, t15204: F, t1570: F, t17536: F, t17539: F, t17542: F, t23: F, t429: F, t17500: F, t3058: F) -> (F, F, F, F, F, F) {
    let t53289 = t11782 * t17515;
    let t53290 = t4297 * t53289;
    let t53293 = t4281 * t9142 * t18187;
    let t53299 = t15083 * t15107;
    let t53327 = t1570 * t15204;
    let t53332 = t17536 * t17539 * t23 * t429 * t17542;
    let t53361 = t3058 * t17500;
    (t53290, t53293, t53299, t53327, t53332, t53361)
}
