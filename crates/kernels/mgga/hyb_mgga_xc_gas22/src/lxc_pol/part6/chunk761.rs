//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 761/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk761<F: Float>(t222: F, t37: F, t4104: F, t2165: F, t3300: F, t251: F, t1347: F, t3316: F, t1346: F, t810: F) -> (F, F, F, F, F, F) {
    let t4106 = t222 * t37 * t4104;
    let t4108 = t2165 - F::cast_from(0.35616666666666666666e-1_f64) * t3300 + F::new(0.53425e-1) * t4106;
    let t4110 = F::new(0.621814e-1) * t4108 * t251;
    let t4112 = F::new(2.0) * t3316 * t1347;
    let t4113 = t1346 * t1346;
    let t4114 = t4113 * t810;
    (t4106, t4108, t4110, t4112, t4113, t4114)
}
