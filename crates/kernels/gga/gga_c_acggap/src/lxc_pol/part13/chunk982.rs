//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 982/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk982<F: Float>(t7637: F, t8555: F, t12610: F, t1426: F, t2297: F, t598: F, t1967: F, t8549: F, t30219: F, t8515: F, t4680: F, t7575: F, t8514: F, t1181: F, t4930: F, t604: F) -> (F, F, F, F, F, F) {
    let t35204 = t7637 * t8555;
    let t35208 = t598 * t1426 * t12610 * t2297;
    let t35210 = t1967 * t8549;
    let t35211 = 0.94344276868812456204e-2 * t35210;
    let t35212 = t30219 * t8515;
    let t35213 = 0.21437009059034868486e-2 * t35212;
    let t35215 = t7575 * t4680 * t8514;
    let t35219 = t7575 * t1181 * t604 * t4930;
    (t35204, t35208, t35211, t35213, t35215, t35219)
}
