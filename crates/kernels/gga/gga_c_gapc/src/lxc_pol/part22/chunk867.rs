//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 867/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk867<F: Float>(t3709: F, t8885: F, t8448: F, t9059: F, t8784: F, t520: F, t8788: F, t2993: F, t3708: F, t9256: F, t1453: F, t435: F) -> (F, F, F, F, F, F, F, F) {
    let t11377 = t3709 * t8885;
    let t11379 = t9059 * t8448;
    let t11380 = t8784 * t11379;
    let t11381 = t520 * t8788;
    let t11382 = t11380 * t11381;
    let t11384 = t2993 * t3708;
    let t11385 = t11384 * t9256;
    let t11387 = t435 * t1453;
    (t11377, t11379, t11380, t11381, t11382, t11384, t11385, t11387)
}
