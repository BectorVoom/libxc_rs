//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 994/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk994<F: Float>(t1998: F, t4720: F, t1298: F, t7380: F, t7381: F, t1524: F, t1983: F, t2095: F, t435: F, t7815: F, t2030: F, t4263: F, t2299: F, t7780: F, t3196: F, t33953: F) -> (F, F, F, F, F, F) {
    let t35403 = t1998 * t4720;
    let t35404 = 0.17149607247227894789e-2 * t35403;
    let t35407 = t7380 * t7381 * t1298;
    let t35408 = t35407 / 32.0;
    let t35410 = t2095 * t1983 * t1524;
    let t35411 = t35410 / 96.0;
    let t35413 = t7815 * t435;
    let t35415 = t2030 * t35413 * t4263;
    let t35418 = t7780 * t2299;
    let t35420 = t33953 * t3196;
    (t35404, t35408, t35411, t35415, t35418, t35420)
}
