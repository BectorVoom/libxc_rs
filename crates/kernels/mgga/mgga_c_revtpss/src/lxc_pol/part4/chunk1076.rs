//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1076/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1076<F: Float>(t14633: F, t14643: F, t14649: F, t14653: F, t14656: F, t14659: F, t1553: F, t1555: F, t227: F, t229: F, t2634: F, t2639: F, t2642: F, t4409: F, t4415: F, t4417: F, t4420: F, t830: F, t833: F) -> (F,) {
    let t14662 = -t14633 * t229 - 24.0 * t14643 * t4417 + 60.0 * t14649 * t4415 - 24.0 * t14653 * t4415 - 12.0 * t14656 * t4415 + 3.0 * t14659 * t227 - 12.0 * t1553 * t2639 + 3.0 * t1553 * t2642 + 3.0 * t1555 * t2634 + 6.0 * t4409 * t833 + 6.0 * t4420 * t830;
    (t14662,)
}
