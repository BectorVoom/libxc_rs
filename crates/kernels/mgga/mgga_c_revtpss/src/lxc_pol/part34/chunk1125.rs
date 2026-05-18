//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1125/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1125<F: Float>(t241: F, t25260: F, t820: F, t72: F, t7778: F, t686: F, t7064: F, t25399: F, t4481: F, t1580: F, t7014: F, t689: F) -> (F, F, F, F, F, F, F) {
    let t27261 = t820 * t25260 * t241;
    let t27278 = t7778 * t72;
    let t27279 = t27278 * t686;
    let t27280 = t7064 * t27279;
    let t27325 = t25399 * t4481;
    let t27334 = t7014 * t1580;
    let t27335 = t689 * t27334;
    (t27261, t27278, t27279, t27280, t27325, t27334, t27335)
}
