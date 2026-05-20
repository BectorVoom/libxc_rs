//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 972/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk972<F: Float>(t23628: F, t24185: F, t1102: F, t11108: F, t198: F, t23562: F, t23564: F, t23567: F, t23570: F, t23571: F, t23651: F, t23665: F, t23698: F, t23769: F, t23772: F, t23816: F, t23818: F, t336: F) -> F {
    let t24186 = t23628 + t24185;
    let t24190 = t1102 * t198 * t24186 * t336 + F::new(2.0) * t11108 * t198 * t23571 * t336 + t23562 - t23564 + t23567 - t23570 - t23651 - t23665 - t23698 - t23769 + t23772 + t23816 + t23818;
    t24190
}
