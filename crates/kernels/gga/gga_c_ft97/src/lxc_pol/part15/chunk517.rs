//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 517/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk517<F: Float>(t2938: F, t5457: F, t898: F, t2946: F, t3738: F, t4967: F, t4971: F, t4975: F, t5242: F, t5245: F) -> (F, F) {
    let t5459 = t898 * t2938 * t5457;
    let t5468 = -0.117377e0 * t5242 + 0.234754e0 * t5245 + t2946 + 0.9628722222222222222e-1 * t3738 - 0.9628722222222222222e-1 * t4967 + 0.28886166666666666666e0 * t4971 - 0.14443083333333333333e0 * t4975;
    (t5459, t5468)
}
