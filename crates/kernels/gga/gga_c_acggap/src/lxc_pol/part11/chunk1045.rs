//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1045/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1045<F: Float>(t2030: F, t4495: F, t7815: F, t2060: F, t5187: F, t4479: F, t142: F, t4099: F, t599: F, t1078: F, t2317: F, t1181: F, t5249: F, t7493: F, t4680: F, t7575: F, t8609: F) -> (F, F, F, F, F, F, F) {
    let t36256 = t2030 * t7815 * t4495;
    let t36259 = t2060 * t7815 * t5187;
    let t36262 = t2060 * t7815 * t4479;
    let t36266 = t2030 * t142 * t599 * t4099;
    let t36269 = t2060 * t1078 * t2317;
    let t36273 = t7493 * t1181 * t599 * t5249;
    let t36274 = 0.10718504529517434243e-2 * t36273;
    let t36276 = t7575 * t4680 * t8609;
    (t36256, t36259, t36262, t36266, t36269, t36274, t36276)
}
