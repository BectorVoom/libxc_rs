//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1754/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1754<F: Float>(t141: F, t3417: F, t89837: F, t1145: F, t89849: F, t89867: F, t89871: F, t89875: F, t43764: F, t89830: F, t6449: F, t3390: F) -> (F, F, F, F, F, F, F, F) {
    let t90402 = t141 * t3417 * t89837;
    let t90405 = t141 * t1145 * t89849;
    let t90408 = t141 * t3417 * t89867;
    let t90411 = t141 * t1145 * t89871;
    let t90414 = t141 * t1145 * t89875;
    let t90417 = t141 * t43764 * t89830;
    let t90419 = t6449 * t6449;
    let t90420 = t3390 * t90419;
    (t90402, t90405, t90408, t90411, t90414, t90417, t90419, t90420)
}
