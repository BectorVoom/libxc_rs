//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 883/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk883<F: Float>(t34622: F, t1165: F, t4718: F, t7351: F, t7426: F, t30543: F, t8469: F, t4521: F, t2268: F, t30797: F, t8473: F, t31419: F, t4810: F, t721: F, t1503: F, t7329: F) -> (F, F, F, F, F, F, F, F) {
    let t34623 = 0.37737710747524982482e-2 * t34622;
    let t34626 = t7426 * t1165 * t7351 * t4718;
    let t34627 = 0.94344276868812456204e-3 * t34626;
    let t34632 = t30543 * t8469;
    let t34633 = 0.18868855373762491241e-1 * t34632;
    let t34636 = t7426 * t1165 * t7351 * t4521;
    let t34638 = t30797 * t2268;
    let t34640 = t30543 * t8473;
    let t34650 = t31419 * t4810 * t721;
    let t34659 = t7329 * t1503;
    (t34623, t34627, t34633, t34636, t34638, t34640, t34650, t34659)
}
