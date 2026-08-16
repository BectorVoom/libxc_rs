//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1301/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1301<F: Float>(t3375: F, t6405: F, t3431: F, t5618: F, t435: F, t6068: F, t6116: F, t997: F, t145: F, t5784: F, t3382: F, t5801: F) -> (F, F, F, F, F, F) {
    let t24173 = t3375 * t6405;
    let t24175 = t3431 * t5618;
    let t24184 = t435 * t6068;
    let t24194 = t997 * t6116;
    let t24196 = t5784 * t145;
    let t24201 = t3382 * t5801;
    (t24173, t24175, t24184, t24194, t24196, t24201)
}
