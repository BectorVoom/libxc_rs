//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1118/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1118<F: Float>(t7447: F, t9663: F, t7440: F, t9734: F, t31773: F, t9660: F, t9730: F, t2030: F, t361: F, t9700: F, t142: F, t5506: F, t599: F) -> (F, F, F, F, F, F) {
    let t39320 = t7447 * t9663;
    let t39322 = t7440 * t9734;
    let t39324 = t31773 * t9660;
    let t39326 = t7447 * t9730;
    let t39330 = t2030 * t361 * t9700;
    let t39334 = t2030 * t142 * t599 * t5506;
    (t39320, t39322, t39324, t39326, t39330, t39334)
}
