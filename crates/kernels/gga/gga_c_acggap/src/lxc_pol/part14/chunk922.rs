//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 922/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk922<F: Float>(t36273: F, t1181: F, t4718: F, t604: F, t7426: F, t31349: F, t3360: F, t4284: F, t2268: F, t30792: F, t7493: F, t7642: F, t8480: F, t30216: F, t8665: F, t30154: F, t36209: F, t7586: F) -> (F, F, F, F, F, F, F) {
    let t36274 = 0.10718504529517434243e-2 * t36273;
    let t36283 = t7426 * t1181 * t604 * t4718;
    let t36284 = 0.42874018118069736972e-3 * t36283;
    let t36286 = t3360 * t31349 * t4284;
    let t36287 = 0.17149607247227894789e-1 * t36286;
    let t36289 = t30792 * t2268;
    let t36292 = t7493 * t8480 * t7642;
    let t36293 = 0.10718504529517434243e-2 * t36292;
    let t36294 = t30216 * t8665;
    let t36299 = t30154 * t7586 * t36209;
    (t36274, t36284, t36287, t36289, t36293, t36294, t36299)
}
