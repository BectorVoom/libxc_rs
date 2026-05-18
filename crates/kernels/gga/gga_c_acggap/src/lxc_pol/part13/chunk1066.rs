//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1066/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1066<F: Float>(t34632: F, t1165: F, t4521: F, t7351: F, t7426: F, t2268: F, t30797: F, t30543: F, t8473: F, t4822: F, t604: F, t8463: F) -> (F, F, F, F, F) {
    let t34633 = F::new(0.18868855373762491241e-1) * t34632;
    let t34636 = t7426 * t1165 * t7351 * t4521;
    let t34638 = t30797 * t2268;
    let t34640 = t30543 * t8473;
    let t34644 = t8463 * t1165 * t604 * t4822;
    (t34633, t34636, t34638, t34640, t34644)
}
