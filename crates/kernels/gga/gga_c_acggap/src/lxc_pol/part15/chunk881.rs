//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 881/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk881<F: Float>(t30937: F, t8602: F, t1165: F, t4718: F, t7351: F, t7426: F, t30543: F, t8469: F, t4521: F, t2268: F, t30797: F, t8473: F, t31419: F, t4810: F, t721: F, t30673: F) -> (F, F, F, F, F, F, F, F) {
    let t34622 = t30937 * t8602;
    let t34626 = t7426 * t1165 * t7351 * t4718;
    let t34632 = t30543 * t8469;
    let t34636 = t7426 * t1165 * t7351 * t4521;
    let t34638 = t30797 * t2268;
    let t34640 = t30543 * t8473;
    let t34650 = t31419 * t4810 * t721;
    let t34655 = 0.34299214494455789578e-2 * t30673;
    (t34622, t34626, t34632, t34636, t34638, t34640, t34650, t34655)
}
