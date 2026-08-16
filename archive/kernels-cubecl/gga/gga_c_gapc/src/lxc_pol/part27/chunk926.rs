//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 926/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk926<F: Float>(t11380: F, t11381: F, t2993: F, t3708: F, t9256: F, t1453: F, t435: F) -> (F, F, F, F) {
    let t11382 = t11380 * t11381;
    let t11384 = t2993 * t3708;
    let t11385 = t11384 * t9256;
    let t11387 = t435 * t1453;
    (t11382, t11384, t11385, t11387)
}
