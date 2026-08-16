//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2600/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2600<F: Float>(t18825: F, t2435: F, t2453: F, t2458: F, t6042: F, t18785: F, t689: F, t779: F, t18316: F, t887: F, t2439: F, t2440: F, t6049: F) -> (F, F, F, F, F) {
    let t61367 = t2435 * t18825;
    let t61371 = t2453 * t6042 * t2458;
    let t61378 = t689 * t779 * t18785;
    let t61385 = t689 * t18316 * t887;
    let t61397 = t2439 * t2440 * t6049;
    (t61367, t61371, t61378, t61385, t61397)
}
