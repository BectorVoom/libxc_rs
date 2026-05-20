//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 903/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk903<F: Float>(t1832: F, t3801: F, t1298: F, t1300: F, t198: F, t336: F, t5023: F, t5062: F, t5065: F, t5067: F, t5070: F, t5107: F, t5111: F, t5189: F, t5191: F, t5194: F, t5196: F, t5200: F, t5204: F, t5209: F, t5501: F) -> (F, F) {
    let t5505 = t1832 * t3801;
    let t5508 = t1300 * t198 * t336 * t5501 - t1298 * t5023 * t5505 - t5062 + t5065 + t5067 - t5070 + t5107 + t5111 + t5189 + t5191 - t5194 - t5196 + t5200 - t5204 - t5209;
    (t5505, t5508)
}
