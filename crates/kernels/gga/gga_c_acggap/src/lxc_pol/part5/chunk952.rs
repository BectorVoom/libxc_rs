//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 952/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk952<F: Float>(t14046: F, t3402: F, t4284: F, t3409: F, t4439: F, t1036: F, t398: F, t429: F, t4347: F, t1487: F, t368: F, t864: F, t1008: F, t4355: F, t1503: F, t3570: F) -> (F, F, F, F, F, F) {
    let t18508 = t14046 * t3402 * t4284;
    let t18510 = t3409 * t4439;
    let t18518 = t1036 * t398 * t429 * t4347;
    let t18523 = t1036 * t398 * t368 * t1487 * t864;
    let t18525 = t1008 * t4355;
    let t18545 = t3570 * t1503;
    (t18508, t18510, t18518, t18523, t18525, t18545)
}
