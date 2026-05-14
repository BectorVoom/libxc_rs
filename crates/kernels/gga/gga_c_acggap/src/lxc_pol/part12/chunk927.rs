//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 927/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk927<F: Float>(t2268: F, t30797: F, t30543: F, t8473: F, t1165: F, t4822: F, t604: F, t8463: F, t2060: F, t507: F, t7811: F, t31419: F, t4810: F, t721: F, t4430: F, t570: F) -> (F, F, F, F, F, F) {
    let t34638 = t30797 * t2268;
    let t34640 = t30543 * t8473;
    let t34644 = t8463 * t1165 * t604 * t4822;
    let t34647 = t2060 * t507 * t7811;
    let t34650 = t31419 * t4810 * t721;
    let t34657 = t570 * t4430;
    (t34638, t34640, t34644, t34647, t34650, t34657)
}
