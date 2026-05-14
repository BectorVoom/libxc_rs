//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 527/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk527<F: Float>(t1502: F, t24: F, t1111: F, t1506: F, t3104: F, t1128: F, t1508: F, t1121: F, t3117: F, t123: F, t438: F) -> (F, F, F, F, F, F, F) {
    let t4314 = t24 * t1502;
    let t4315 = t1111 * t4314;
    let t4327 = t3104 * t1506;
    let t4333 = t1128 * t1508;
    let t4334 = t1121 * t4333;
    let t4336 = t3117 * t1506;
    let t4356 = t123 * t438;
    (t4314, t4315, t4327, t4333, t4334, t4336, t4356)
}
