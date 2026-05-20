//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1001/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1001<F: Float>(t11217: F, t12189: F, t1100: F, t1102: F, t11105: F, t11108: F, t11114: F, t11118: F, t11398: F, t11530: F, t11533: F, t11547: F, t11608: F, t11612: F, t11614: F, t11618: F, t198: F, t3329: F, t3336: F, t336: F, t5023: F) -> F {
    let t12190 = t11217 + t12189;
    let t12198 = -F::new(3.0) * t1100 * t3329 * t3336 * t5023 + t1102 * t12190 * t198 * t336 + F::new(2.0) * t11105 * t11108 * t198 * t336 - t11114 + t11118 - t11398 - t11530 + t11533 - t11547 + t11608 - t11612 + t11614 - t11618;
    t12198
}
