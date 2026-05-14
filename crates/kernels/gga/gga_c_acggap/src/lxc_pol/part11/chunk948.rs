//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 948/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk948<F: Float>(t34626: F, t1181: F, t4818: F, t599: F, t8463: F, t30543: F, t8469: F, t1165: F, t4521: F, t7351: F, t7426: F, t2268: F, t30797: F, t8473: F, t4822: F, t604: F) -> (F, F, F, F, F, F, F) {
    let t34627 = 0.94344276868812456204e-3 * t34626;
    let t34630 = t8463 * t1181 * t599 * t4818;
    let t34632 = t30543 * t8469;
    let t34633 = 0.18868855373762491241e-1 * t34632;
    let t34636 = t7426 * t1165 * t7351 * t4521;
    let t34638 = t30797 * t2268;
    let t34640 = t30543 * t8473;
    let t34644 = t8463 * t1165 * t604 * t4822;
    (t34627, t34630, t34633, t34636, t34638, t34640, t34644)
}
