//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1198/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1198<F: Float>(t4424: F, t827: F, t828: F, t1559: F, t221: F, t2485: F, t2484: F, t1544: F, t775: F) -> (F, F, F, F) {
    let t4426 = t827 * t828 * t4424;
    let t4430 = t2485 * t221 * t1559;
    let t4431 = t2484 * t4430;
    let t4433 = t1544 * t775;
    (t4426, t4430, t4431, t4433)
}
