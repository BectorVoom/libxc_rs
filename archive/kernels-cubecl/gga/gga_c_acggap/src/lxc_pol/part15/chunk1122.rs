//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1122/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1122<F: Float>(t31346: F, t5932: F, t7433: F, t9637: F, t1773: F, t2030: F, t2031: F, t1181: F, t5537: F, t7351: F, t7564: F, t5796: F, t7822: F) -> (F, F, F, F, F) {
    let t39391 = t31346 * t5932;
    let t39393 = t7433 * t9637;
    let t39402 = t2030 * t1773 * t2031;
    let t39406 = t7564 * t1181 * t7351 * t5537;
    let t39412 = t7822 * t5796;
    (t39391, t39393, t39402, t39406, t39412)
}
