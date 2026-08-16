//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1075/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1075<F: Float>(t5920: F, t94: F, t21663: F, t38: F, t5868: F, t76: F, t1470: F, t4173: F, t1493: F, t1497: F, t77: F, t5872: F, t84: F) -> (F, F, F, F, F, F) {
    let t29508 = t94 * t5920;
    let t29513 = t21663 * t38;
    let t29532 = t76 * t5868;
    let t29538 = t4173 * t1470;
    let t29543 = t1493 * t1497;
    let t29544 = t77 * t29543;
    let t29547 = t84 * t5872;
    (t29508, t29513, t29532, t29538, t29544, t29547)
}
