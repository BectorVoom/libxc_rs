//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 336/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk336<F: Float>(t1069: F, t1072: F, t1444: F, t1451: F, t1454: F, t1457: F) -> F {
    let t1471 = F::new(0.3529725e1) * t1451 - t1069 - F::new(0.516475e0) * t1444 + F::new(0.6311625e0) * t1454 - t1072 - F::new(0.104195e0) * t1457;
    t1471
}
