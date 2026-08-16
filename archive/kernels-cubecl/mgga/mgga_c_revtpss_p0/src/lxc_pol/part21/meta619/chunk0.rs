//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2374/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2374<F: Float>(t10832: F, t10845: F, t820: F, t823: F, t9948: F, t839: F, t10639: F, t221: F, t2484: F, t2485: F, t10820: F, t2652: F) -> (F, F, F, F, F) {
    let t40357 = t10845 * t10832;
    let t40360 = t820 * t823 * t9948;
    let t40361 = t40360 * t839;
    let t40365 = t2484 * t2485 * t221 * t10639;
    let t40367 = t2652 * t10820;
    (t40357, t40360, t40361, t40365, t40367)
}
