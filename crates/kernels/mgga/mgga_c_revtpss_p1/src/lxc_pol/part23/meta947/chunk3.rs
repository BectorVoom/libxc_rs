//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3131/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3131<F: Float>(t81317: F, t81322: F, t81326: F, t81328: F, t81330: F, t81333: F, t81336: F, t81338: F, t81341: F, t81343: F, t81352: F, t81558: F, t81560: F, t81562: F, t81566: F, t81570: F, t81573: F, t81575: F, t81577: F, t81580: F) -> F {
    let t82386 = t81317 - t81322 - t81326 + t81328 + t81330 + t81333 - t81336 - t81338 - t81341 - t81343 + t81352 + t81558 + t81560 - t81562 + t81566 + t81570 + t81573 + t81575 + t81577 - t81580;
    t82386
}
