//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 906/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk906<F: Float>(t50: F, t6554: F, t1018: F, t1020: F, t1226: F, t1228: F, t208: F, t2828: F, t2832: F, t3294: F, t3298: F, t367: F, t368: F, t501: F, t502: F, t8399: F, t8406: F, t9345: F, zeta_threshold: F) -> (F, F) {
    let t51 = t50 <= zeta_threshold;
    let t9352 = piecewise3(t51, 0.0, t6554);
    let t9356 = t208 * (t8399 * t368 / 2.0 + 3.0 / 2.0 * t2828 * t1020 + 3.0 / 2.0 * t1018 * t2832 + t367 * t8406 / 2.0 + t9345 * t502 / 2.0 + 3.0 / 2.0 * t3294 * t1228 + 3.0 / 2.0 * t1226 * t3298 + t501 * t9352 / 2.0);
    (t9352, t9356)
}
