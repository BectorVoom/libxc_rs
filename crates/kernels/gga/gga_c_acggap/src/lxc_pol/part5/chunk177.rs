//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 177/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk177<F: Float>(t50: F, t478: F, t52: F, t477: F, t59: F, zeta_threshold: F) -> (F,) {
    let t51 = t50 <= zeta_threshold;
    let t481 = piecewise3(t51, 0.0, 4.0 / 3.0 * t52 * t478);
    let t483 = (t477 + t481) * t59;
    (t483,)
}
