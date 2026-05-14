//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 982/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk982<F: Float>(t1046: F, t12519: F, t3445: F, t3488: F, t17444: F, t47377: F, t5400: F, t639: F, t47766: F, t7115: F, t7505: F, t41039: F, t41042: F, t3390: F, t16824: F, t186: F, t211: F) -> (F, F, F, F, F, F, F) {
    let t47784 = 8.0 / 15.0 * t12519 * t1046;
    let t47786 = 4.0 / 5.0 * t3488 * t3445;
    let t47790 = 128.0 / 27.0 * t639 * t5400 * t17444 * t47377;
    let t47793 = 32.0 / 15.0 * t7115 * t7505 * t47766;
    let t47794 = 64.0 / 45.0 * t41039;
    let t47795 = 64.0 / 45.0 * t41042;
    let t47796 = t3390 * t3390;
    let t47800 = 16.0 / 5.0 * t211 * t186 * t16824 * t47796;
    (t47784, t47786, t47790, t47793, t47794, t47795, t47800)
}
