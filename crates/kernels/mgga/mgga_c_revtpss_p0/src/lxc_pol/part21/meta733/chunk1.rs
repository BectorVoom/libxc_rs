//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2581/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2581<F: Float>(t39494: F, t3964: F, t4096: F, t40270: F, t4089: F, t138: F, t2438: F, t4131: F, t9674: F, t1444: F, t2782: F, t4075: F, t556: F) -> (F, F, F, F) {
    let t47454 = F::cast_from(0.20561456923286030469e-1_f64) * t3964 * t4096 * t39494;
    let t47455 = t40270 * t4089;
    let t47466 = t9674 * t138 * t2438 * t4131;
    let t47472 = t2782 * t556 * t4075 * t1444 * t4131;
    (t47454, t47455, t47466, t47472)
}
