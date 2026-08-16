//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3278/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3278<F: Float>(t39419: F, t39422: F, t39429: F, t39432: F, t39442: F, t61019: F, t61021: F, t61022: F, t61026: F, t61027: F, t61028: F, t61029: F, t61031: F, t61032: F, t61039: F, t61088: F, t61091: F, t61094: F, t61097: F) -> F {
    let t62259 = t61019 - t39419 - t39422 + t61021 - t61022 - t61026 - t61027 - t39429 - t39432 + t61028 + t61029 + t61031 + t39442 + t61032 + t61039 + t61088 + t61091 - t61094 + t61097;
    t62259
}
