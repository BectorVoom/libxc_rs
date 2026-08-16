//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1060/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1060<F: Float>(t3014: F, t6205: F, t2926: F, t6141: F, t342: F, t6343: F, t6271: F, t73: F, t11249: F, t6305: F) -> (F, F, F, F, F) {
    let t19303 = t6205 * t3014;
    let t19330 = t6141 * t2926;
    let t19351 = t342 * t6343;
    let t19446 = t6271 * t73;
    let t19450 = t6305 * t11249;
    (t19303, t19330, t19351, t19446, t19450)
}
