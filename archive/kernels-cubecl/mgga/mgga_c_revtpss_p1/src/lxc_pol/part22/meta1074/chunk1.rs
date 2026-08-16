//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3853/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3853<F: Float>(t39773: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t47003: F, t47059: F, t73389: F, t73390: F, t73398: F, t73399: F, t73402: F, t73403: F, t73411: F, t73412: F, t73416: F, t73418: F) -> F {
    let t74103 = t47003 + t73389 + t73390 + t39773 + t73398 - t73399 - t39783 - t39786 - t39791 - t39795 + t73402 - t73403 + t73411 - t73412 - t73416 + t39799 + t47059 + t73418;
    t74103
}
