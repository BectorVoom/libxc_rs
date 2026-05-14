//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 999/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk999<F: Float>(t6351: F, t7647: F, t1165: F, t34278: F, t5641: F, t604: F, t34368: F, t34369: F, t5693: F, t34691: F, t34692: F, t5697: F, t137: F, t336: F, t578: F, t6119: F) -> (F, F, F, F, F) {
    let t39686 = t7647 * t6351;
    let t39690 = t34278 * t1165 * t604 * t5641;
    let t39693 = t34368 * t34369 * t5693;
    let t39696 = t34691 * t34692 * t5697;
    let t39700 = t578 * t336 * t6119 * t137;
    (t39686, t39690, t39693, t39696, t39700)
}
