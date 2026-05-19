//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 603/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk603<F: Float>(t3651: F, t3652: F, t3636: F, t3641: F, t3647: F, t209: F, t1049: F, t2964: F) -> (F, F, F, F) {
    let t3653 = t3651 * t3652;
    let t3655 = F::cast_from(0.60736713313768998074e-4_f64) * t3636 - F::cast_from(0.43449121406768801912e-4_f64) * t3641 - F::cast_from(0.12653481940368541265e-5_f64) * t3647 + F::cast_from(0.27155700879230501195e-5_f64) * t3653;
    let t3656 = t3655 * t209;
    let t3658 = F::new(2.0) * t2964 * t1049;
    let t3659 = t1049 * t1049;
    (t3655, t3656, t3658, t3659)
}
