//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1172/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1172<F: Float>(t3354: F, t9793: F, t1528: F, t47733: F, t12355: F, t2485: F, t47372: F, t478: F, t48542: F, t48544: F, t48546: F, t48548: F, t48550: F, t48552: F) -> (F, F, F, F, F) {
    let t48554 = t9793 * t3354;
    let t48556 = t1528 * t47733;
    let t48558 = t2485 * t12355;
    let t48560 = t478 * t47372;
    let t48562 = -F::new(28.0) / F::new(81.0) * t48542 + F::new(8.0) / F::new(9.0) * t48544 - t48546 / F::new(3.0) - F::new(4.0) / F::new(9.0) * t48548 + t48550 / F::new(3.0) - F::new(28.0) / F::new(81.0) * t48552 + F::new(8.0) / F::new(9.0) * t48554 - t48556 / F::new(3.0) - F::new(4.0) / F::new(9.0) * t48558 + t48560 / F::new(3.0);
    (t48554, t48556, t48558, t48560, t48562)
}
