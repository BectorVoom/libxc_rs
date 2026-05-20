//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1787/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1787<F: Float>(t1450: F, t6816: F, t6836: F, t196: F, t197: F, t6773: F, t5920: F, t94: F, t21663: F, t38: F, t5868: F, t76: F) -> (F, F, F, F, F, F) {
    let t29494 = t1450 * t6816;
    let t29498 = t1450 * t6836;
    let t29506 = t6773 * t196 * t197;
    let t29508 = t94 * t5920;
    let t29513 = t21663 * t38;
    let t29532 = t76 * t5868;
    (t29494, t29498, t29506, t29508, t29513, t29532)
}
