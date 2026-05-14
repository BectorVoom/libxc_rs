//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 597/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk597<F: Float>(t223: F, t9542: F, t13411: F, t4952: F, t6783: F, t2455: F, t3780: F, t1127: F, t2427: F, t677: F, t25: F, t3817: F, t3762: F, t1113: F, t122: F, t1095: F, t2380: F) -> (F, F, F, F, F, F, F, F) {
    let t13412 = t9542 * t223;
    let t13413 = t13411 * t13412;
    let t13414 = t6783 * t4952;
    let t13417 = t3780 * t2455;
    let t13421 = t2427 * t1127;
    let t13422 = t677 * t13421;
    let t13425 = t3817 * t25;
    let t13426 = t13425 * t3762;
    let t13429 = t1113 * t122;
    let t13433 = t1095 * t2380;
    (t13412, t13413, t13414, t13417, t13422, t13426, t13429, t13433)
}
