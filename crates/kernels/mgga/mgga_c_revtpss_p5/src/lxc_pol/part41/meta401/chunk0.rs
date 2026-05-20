//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1374/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1374<F: Float>(t1248: F, t3604: F, t6688: F, t3720: F, t20266: F, t5312: F, t17475: F, t20293: F, t20318: F, t5308: F, t20310: F, t20306: F) -> (F, F, F, F, F, F) {
    let t21119 = t3604 * t1248;
    let t21120 = t6688 * t21119;
    let t21121 = t3720 * t21120;
    let t21126 = t5312 * t20266;
    let t21129 = t17475 * t20293;
    let t21134 = t5308 * t20318;
    let t21137 = t5308 * t20310;
    let t21140 = t5308 * t20306;
    (t21121, t21126, t21129, t21134, t21137, t21140)
}
