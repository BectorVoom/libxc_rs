//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 984/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk984<F: Float>(t2030: F, t6300: F, t7815: F, t6304: F, t6309: F, t570: F, t6279: F, t7447: F, t9663: F, t7440: F, t9734: F, t31773: F, t9660: F, t9730: F, t361: F, t9700: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39308 = t2030 * t7815 * t6300;
    let t39311 = t2030 * t7815 * t6304;
    let t39314 = t2030 * t7815 * t6309;
    let t39318 = t570 * t6279;
    let t39320 = t7447 * t9663;
    let t39322 = t7440 * t9734;
    let t39324 = t31773 * t9660;
    let t39326 = t7447 * t9730;
    let t39330 = t2030 * t361 * t9700;
    (t39308, t39311, t39314, t39318, t39320, t39322, t39324, t39326, t39330)
}
