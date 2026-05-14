//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1079/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1079<F: Float>(t127349: F, t127357: F, t127359: F, t127361: F, t127363: F, t127366: F, t127369: F, t127371: F, t127373: F, t127375: F, t127378: F, t129491: F, t129502: F, t569: F, t5787: F, t8761: F) -> (F,) {
    let t129507 = t127349 - t127357 - t127359 + t127361 + (2.0 * t129491 + t129502) * t569 + t8761 * t5787 - 2.0 * t127363 - t127366 - t127369 - t127371 - t127373 - t127375 - t127378;
    (t129507,)
}
